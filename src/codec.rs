use chessy::{Piece, PieceType};
use rand::make_rng;

pub fn index_to_str(index: usize) -> String {
    let (row, col) = index_to_pos(index);
    row_to_char(row).to_string() + &col_to_char(col).to_string()
}

pub fn str_to_pos(string: String) -> (u32, u32) {
    let vec = Vec::from_iter(string.chars());
    (char_to_row(vec[1]), char_to_col(vec[0]))
}

pub fn str_to_promotion_piece(string: &str) -> Option<PieceType> {
    match string.to_lowercase().as_str() {
        "p" => Some(PieceType::Pawn),
        "n" => Some(PieceType::Knight),
        "q" => Some(PieceType::Queen),
        "r" => Some(PieceType::Rook),
        "b" => Some(PieceType::Bishop),
        "-" => None,
        _ => panic!("ABAAAAAAAAAAA"),
    }
}

pub fn str_to_piece_type(string: &str) -> Option<PieceType> {
    match string.to_lowercase().as_str() {
        "p" => Some(PieceType::Pawn),
        "n" => Some(PieceType::Knight),
        "q" => Some(PieceType::Queen),
        "r" => Some(PieceType::Rook),
        "b" => Some(PieceType::Bishop),
        "p" => Some(PieceType::Pawn),
        "k" => Some(PieceType::King),
        "-" => None,
        " " => None,
        _ => panic!("ABAAAAAAAAAAA"),
    }
}

pub fn piece_to_char(piece: Option<Piece>) -> char {
    if let Some(p) = piece {
        match p.color {
            chessy::Color::Black => piece_type_to_char(Some(p.piece_type)),
            chessy::Color::White => piece_type_to_char(Some(p.piece_type)).to_ascii_uppercase(),
        }
    } else {
        ' '
    }
}

pub fn board_to_string(board: [Option<Piece>; 64]) -> String {
    board.iter().fold(String::new(), |a, piece| {
        a + &piece_to_char(*piece).to_string()
    })
}

pub fn char_to_piece(piece: char) -> Option<Piece> {
    if let Some(piece_type) = str_to_piece_type(&piece.to_string()) {
        Some(Piece {
            piece_type: piece_type,
            color: match piece.is_uppercase() {
                true => chessy::Color::White,
                false => chessy::Color::Black,
            },
        })
    } else {
        None
    }
}

pub fn piece_type_to_char(piece_type: Option<PieceType>) -> char {
    match piece_type {
        Some(PieceType::Bishop) => 'b',
        Some(PieceType::King) => 'k',
        Some(PieceType::Knight) => 'n',
        Some(PieceType::Pawn) => 'p',
        Some(PieceType::Queen) => 'q',
        Some(PieceType::Rook) => 'r',
        None => '-',
    }
}

pub fn index_to_pos(i: usize) -> (u32, u32) {
    (i as u32 / 8, (i as u32) % 8)
}

pub fn pos_to_index(pos: (u32, u32)) -> usize {
    (pos.1 * 8 + pos.0) as usize
}

pub fn bound_check(pos: (u32, u32)) -> Option<(u32, u32)> {
    if pos.1 <= 7 && pos.0 <= 7 {
        Some(pos)
    } else {
        None
    }
}

pub fn row_to_char(row: u32) -> char {
    match row {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => {
            panic!("AAAAAAAAAAAAAAAAA")
        }
    }
}

pub fn col_to_char(row: u32) -> char {
    match row {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        _ => {
            panic!("AAAAAAAAAAAAAAAAA")
        }
    }
}

pub fn char_to_col(col: char) -> u32 {
    match col {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => {
            panic!("AAAAAAAAAAAAAAAAA")
        }
    }
}

pub fn char_to_row(col: char) -> u32 {
    match col {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        _ => {
            panic!("AAAAAAAAAAAAAAAAA")
        }
    }
}
