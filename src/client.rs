use std::{
    io::{
        self, BufRead, BufReader, BufWriter, Error,
        ErrorKind::{self, WouldBlock},
        Read, Write,
    },
    net::{TcpListener, TcpStream},
    ptr::read,
};

use chessy::{self, Chess};
use rand::{random, random_bool};

pub struct MultiplayerChess {
    pub setting_up: bool,
    pub chess: Chess,
    pub player_color: chessy::Color,
    pub is_server: bool,
    stream: TcpStream,
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
            chessy::Color::White
        } else {
            chessy::Color::Black
        };

        stream
            .write_all(
                match you {
                    chessy::Color::Black => "B\nW\n",
                    chessy::Color::White => "B\nW\n",
                }
                .as_bytes(),
            )
            .unwrap();
        stream.flush().unwrap();

        let chess = MultiplayerChess {
            stream: stream,
            chess: chesss,
            setting_up: false,
            player_color: you,
            is_server: true,
        };

        return chess;
    }

    fn new_from_client(endpoint: String) -> MultiplayerChess {
        let mut stream = TcpStream::connect(endpoint).unwrap();

        let mut buff = String::new();
        stream.set_nonblocking(true).unwrap();
        let _ = stream.read_to_string(&mut buff); // ignore this too
        println!("adasd");
        println!("{}", buff);

        let chess = MultiplayerChess {
            chess: chessy::Chess::new(),
            setting_up: true,
            player_color: chessy::Color::White,
            is_server: false,
            stream,
        };

        return chess;
    }

    pub fn tick(&mut self) {
        let mut buffer: String = String::new();

        let result = BufReader::new(&self.stream).read_line(&mut buffer);

        match result {
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                return;
            }
            Err(r) => Err(r).unwrap(),
            _ => {}
        }

        println!("{}", buffer);

        if self.setting_up {
            if buffer.trim() == "W" {
                self.player_color = chessy::Color::Black;
                self.setting_up = false;
            } else if buffer.trim() == "B" {
                self.player_color = chessy::Color::White;
                self.setting_up = false;
            }
        }
    }
}

pub fn create_server_game<'a>() -> Result<MultiplayerChess, ()> {
    return Ok(MultiplayerChess::new_from_server());
}

pub fn create_client_game<'a>(endpoint: String) -> Result<MultiplayerChess, ()> {
    Ok(MultiplayerChess::new_from_client(endpoint))
}
