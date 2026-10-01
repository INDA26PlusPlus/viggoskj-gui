use crate::codec::*;
use chessy::{self, Chess};
use rand::{random, random_bool};
use std::{
    io::{
        self, BufRead, BufReader, BufWriter, Error,
        ErrorKind::{self, WouldBlock},
        Read, Write,
    },
    mem::transmute,
    net::{TcpListener, TcpStream},
    ptr::read,
    sync::mpsc::RecvTimeoutError,
};

pub struct MultiplayerChess {
    pub setting_up: bool,
    pub chess: Chess,
    pub player_color: chessy::Color,
    pub is_server: bool,
    old_chess: Chess,
    stream: TcpStream,
    your_turn: bool,
}

impl MultiplayerChess {
    fn new_from_server<'a>() -> MultiplayerChess {
        let listener = TcpListener::bind("127.0.0.1:6767").unwrap();

        println!("waiting for connection.");

        let mut stream = listener.accept().unwrap().0;

        println!("connected.");

        stream.set_nonblocking(true).unwrap();

        let chesss = chessy::Chess::new();

        let you = if random_bool(0.5f64) {
            println!("you are white");
            chessy::Color::White
        } else {
            println!("you are black");
            chessy::Color::Black
        };

        let mut chess = MultiplayerChess {
            stream: stream,
            chess: chesss,
            setting_up: false,
            player_color: you,
            is_server: true,
            old_chess: chesss,
            your_turn: match you {
                chessy::Color::Black => false,
                chessy::Color::White => true,
            },
        };

        chess.send_data(
            match you {
                chessy::Color::Black => "B\nW\n",
                chessy::Color::White => "W\nB\n",
            }
            .to_string(),
        );

        return chess;
    }

    fn new_from_client(endpoint: String) -> MultiplayerChess {
        let mut stream = TcpStream::connect(endpoint).unwrap();

        let mut buff = String::new();
        stream.set_nonblocking(true).unwrap();
        let _ = stream.read_to_string(&mut buff); // ignore this too
        println!("{}", buff);

        let chess = MultiplayerChess {
            chess: chessy::Chess::new(),
            setting_up: true,
            player_color: chessy::Color::White,
            old_chess: chessy::Chess::new(),
            is_server: false,
            your_turn: false,
            stream,
        };

        return chess;
    }

    pub fn send_data(&mut self, data: String) {
        println!("Sent: \"{}\"", data);
        self.stream.write_all((data + "\n").as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn move_piece(
        &mut self,
        from: usize,
        to: usize,
        promotion: Option<chessy::PieceType>,
    ) -> Result<(), chessy::Error> {
        if self.your_turn && self.chess.turn == self.player_color {
            self.old_chess = self.chess.clone();
            self.chess.move_piece(from, to, promotion)?;

            let mut move_str = String::new();
            move_str += &index_to_str(from);
            move_str += &index_to_str(to);
            move_str += &piece_type_to_char(promotion).to_string();
            move_str += &board_to_string(self.chess.board);
            move_str += "\n";

            self.send_data(move_str);
            self.your_turn = false;
            Ok(())
        } else {
            Err(chessy::Error::IllegalMove)
        }
    }

    pub fn tick(&mut self) {
        let mut buffer: String = String::new();

        let result = BufReader::new(&self.stream).read_line(&mut buffer);

        buffer = buffer.to_string();

        match result {
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                return;
            }
            Err(r) => Err(r).unwrap(),
            _ => {}
        }

        println!("Recived: \"{}\"", buffer);

        if self.setting_up {
            if buffer.trim() == "W" {
                println!("you are black");
                self.player_color = chessy::Color::Black;
                self.setting_up = false;
                return;
            } else if buffer.trim() == "B" {
                println!("you are white");
                self.player_color = chessy::Color::White;
                self.setting_up = false;
                self.your_turn = true;
                return;
            }
        }

        if self.your_turn {
            match self.chess.game_status() {
                chessy::GameStatus::Checkmate => {
                    self.send_data("CHECKMATE".to_string());
                }
                chessy::GameStatus::Stalemate => {
                    self.send_data("STALEMATE".to_string());
                }
                _ => {}
            }
        }

        if buffer.len() == 4 + 1 + 64 + 1 {
            println!("parsing move");

            let (from_str, rest_str) = buffer.split_at(2);
            let (to_str, rest_str) = rest_str.split_at(2);
            let (promotion_str, board_str) = rest_str.split_at(1);

            let from = pos_to_index(str_to_pos(from_str.to_string()));
            let to: usize = pos_to_index(str_to_pos(to_str.to_string()));
            let promotion = str_to_promotion_piece(promotion_str);

            let reject = match self.chess.move_piece(from, to, promotion) {
                Err(err) => true,
                Ok(()) => board_to_string(self.chess.board).trim() != board_str.trim(),
            };

            if reject {
                self.send_data("REJECT".to_string());
            } else {
                self.your_turn = true;
                self.send_data("OK".to_string());
            }

            return;
        }

        if buffer.trim() == "OK" {
            self.your_turn = false;
        }

        if buffer.trim() == "REJECT" {
            self.chess = self.old_chess;
            self.your_turn = true;
        }
    }
}

pub fn create_server_game() -> Result<MultiplayerChess, ()> {
    return Ok(MultiplayerChess::new_from_server());
}

pub fn create_client_game(endpoint: String) -> Result<MultiplayerChess, ()> {
    Ok(MultiplayerChess::new_from_client(endpoint))
}
