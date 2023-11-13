#![allow(dead_code)]
use std::collections::HashSet;
use color_eyre::eyre::Result;
use colored::{Colorize, ColoredString};

mod magic_numbers;
use magic_numbers::{
    SQUARES_TO_EDGE,
    DIRECTION_OFFSETS,
    KNIGHT_XY_OFFSETS,
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Coordinate {
    A1, A2, A3, A4, A5, A6, A7, A8,
    B1, B2, B3, B4, B5, B6, B7, B8,
    C1, C2, C3, C4, C5, C6, C7, C8,
    D1, D2, D3, D4, D5, D6, D7, D8,
    E1, E2, E3, E4, E5, E6, E7, E8,
    F1, F2, F3, F4, F5, F6, F7, F8,
    G1, G2, G3, G4, G5, G6, G7, G8,
    H1, H2, H3, H4, H5, H6, H7, H8,
}
impl Coordinate {
    pub fn from(coordinate: &str) -> Result<Option<Self>, String> {
        match coordinate {// {{{
            "a1" => Ok(Some(Coordinate::A1)),
            "a2" => Ok(Some(Coordinate::A2)),
            "a3" => Ok(Some(Coordinate::A3)),
            "a4" => Ok(Some(Coordinate::A4)),
            "a5" => Ok(Some(Coordinate::A5)),
            "a6" => Ok(Some(Coordinate::A6)),
            "a7" => Ok(Some(Coordinate::A7)),
            "a8" => Ok(Some(Coordinate::A8)),
            "b1" => Ok(Some(Coordinate::B1)),
            "b2" => Ok(Some(Coordinate::B2)),
            "b3" => Ok(Some(Coordinate::B3)),
            "b4" => Ok(Some(Coordinate::B4)),
            "b5" => Ok(Some(Coordinate::B5)),
            "b6" => Ok(Some(Coordinate::B6)),
            "b7" => Ok(Some(Coordinate::B7)),
            "b8" => Ok(Some(Coordinate::B8)),
            "c1" => Ok(Some(Coordinate::C1)),
            "c2" => Ok(Some(Coordinate::C2)),
            "c3" => Ok(Some(Coordinate::C3)),
            "c4" => Ok(Some(Coordinate::C4)),
            "c5" => Ok(Some(Coordinate::C5)),
            "c6" => Ok(Some(Coordinate::C6)),
            "c7" => Ok(Some(Coordinate::C7)),
            "c8" => Ok(Some(Coordinate::C8)),
            "d1" => Ok(Some(Coordinate::D1)),
            "d2" => Ok(Some(Coordinate::D2)),
            "d3" => Ok(Some(Coordinate::D3)),
            "d4" => Ok(Some(Coordinate::D4)),
            "d5" => Ok(Some(Coordinate::D5)),
            "d6" => Ok(Some(Coordinate::D6)),
            "d7" => Ok(Some(Coordinate::D7)),
            "d8" => Ok(Some(Coordinate::D8)),
            "e1" => Ok(Some(Coordinate::E1)),
            "e2" => Ok(Some(Coordinate::E2)),
            "e3" => Ok(Some(Coordinate::E3)),
            "e4" => Ok(Some(Coordinate::E4)),
            "e5" => Ok(Some(Coordinate::E5)),
            "e6" => Ok(Some(Coordinate::E6)),
            "e7" => Ok(Some(Coordinate::E7)),
            "e8" => Ok(Some(Coordinate::E8)),
            "f1" => Ok(Some(Coordinate::F1)),
            "f2" => Ok(Some(Coordinate::F2)),
            "f3" => Ok(Some(Coordinate::F3)),
            "f4" => Ok(Some(Coordinate::F4)),
            "f5" => Ok(Some(Coordinate::F5)),
            "f6" => Ok(Some(Coordinate::F6)),
            "f7" => Ok(Some(Coordinate::F7)),
            "f8" => Ok(Some(Coordinate::F8)),
            "g1" => Ok(Some(Coordinate::G1)),
            "g2" => Ok(Some(Coordinate::G2)),
            "g3" => Ok(Some(Coordinate::G3)),
            "g4" => Ok(Some(Coordinate::G4)),
            "g5" => Ok(Some(Coordinate::G5)),
            "g6" => Ok(Some(Coordinate::G6)),
            "g7" => Ok(Some(Coordinate::G7)),
            "g8" => Ok(Some(Coordinate::G8)),
            "h1" => Ok(Some(Coordinate::H1)),
            "h2" => Ok(Some(Coordinate::H2)),
            "h3" => Ok(Some(Coordinate::H3)),
            "h4" => Ok(Some(Coordinate::H4)),
            "h5" => Ok(Some(Coordinate::H5)),
            "h6" => Ok(Some(Coordinate::H6)),
            "h7" => Ok(Some(Coordinate::H7)),
            "h8" => Ok(Some(Coordinate::H8)),
            "-"  => Ok(None),
            _    => Err("Invalid FEN: Invalid en passant coordinate".to_string()),
        }// }}}
    }
    pub fn to_string(&self) -> String {
        match self {// {{{
            Coordinate::A1 => "a1".to_string(),
            Coordinate::A2 => "a2".to_string(),
            Coordinate::A3 => "a3".to_string(),
            Coordinate::A4 => "a4".to_string(),
            Coordinate::A5 => "a5".to_string(),
            Coordinate::A6 => "a6".to_string(),
            Coordinate::A7 => "a7".to_string(),
            Coordinate::A8 => "a8".to_string(),
            Coordinate::B1 => "b1".to_string(),
            Coordinate::B2 => "b2".to_string(),
            Coordinate::B3 => "b3".to_string(),
            Coordinate::B4 => "b4".to_string(),
            Coordinate::B5 => "b5".to_string(),
            Coordinate::B6 => "b6".to_string(),
            Coordinate::B7 => "b7".to_string(),
            Coordinate::B8 => "b8".to_string(),
            Coordinate::C1 => "c1".to_string(),
            Coordinate::C2 => "c2".to_string(),
            Coordinate::C3 => "c3".to_string(),
            Coordinate::C4 => "c4".to_string(),
            Coordinate::C5 => "c5".to_string(),
            Coordinate::C6 => "c6".to_string(),
            Coordinate::C7 => "c7".to_string(),
            Coordinate::C8 => "c8".to_string(),
            Coordinate::D1 => "d1".to_string(),
            Coordinate::D2 => "d2".to_string(),
            Coordinate::D3 => "d3".to_string(),
            Coordinate::D4 => "d4".to_string(),
            Coordinate::D5 => "d5".to_string(),
            Coordinate::D6 => "d6".to_string(),
            Coordinate::D7 => "d7".to_string(),
            Coordinate::D8 => "d8".to_string(),
            Coordinate::E1 => "e1".to_string(),
            Coordinate::E2 => "e2".to_string(),
            Coordinate::E3 => "e3".to_string(),
            Coordinate::E4 => "e4".to_string(),
            Coordinate::E5 => "e5".to_string(),
            Coordinate::E6 => "e6".to_string(),
            Coordinate::E7 => "e7".to_string(),
            Coordinate::E8 => "e8".to_string(),
            Coordinate::F1 => "f1".to_string(),
            Coordinate::F2 => "f2".to_string(),
            Coordinate::F3 => "f3".to_string(),
            Coordinate::F4 => "f4".to_string(),
            Coordinate::F5 => "f5".to_string(),
            Coordinate::F6 => "f6".to_string(),
            Coordinate::F7 => "f7".to_string(),
            Coordinate::F8 => "f8".to_string(),
            Coordinate::G1 => "g1".to_string(),
            Coordinate::G2 => "g2".to_string(),
            Coordinate::G3 => "g3".to_string(),
            Coordinate::G4 => "g4".to_string(),
            Coordinate::G5 => "g5".to_string(),
            Coordinate::G6 => "g6".to_string(),
            Coordinate::G7 => "g7".to_string(),
            Coordinate::G8 => "g8".to_string(),
            Coordinate::H1 => "h1".to_string(),
            Coordinate::H2 => "h2".to_string(),
            Coordinate::H3 => "h3".to_string(),
            Coordinate::H4 => "h4".to_string(),
            Coordinate::H5 => "h5".to_string(),
            Coordinate::H6 => "h6".to_string(),
            Coordinate::H7 => "h7".to_string(),
            Coordinate::H8 => "h8".to_string(),
        }// }}}
    }
    pub fn into_index(self) -> usize {
        match self {// {{{
            Coordinate::A8 => 0,
            Coordinate::B8 => 1,
            Coordinate::C8 => 2,
            Coordinate::D8 => 3,
            Coordinate::E8 => 4,
            Coordinate::F8 => 5,
            Coordinate::G8 => 6,
            Coordinate::H8 => 7,
            Coordinate::A7 => 8,
            Coordinate::B7 => 9,
            Coordinate::C7 => 10,
            Coordinate::D7 => 11,
            Coordinate::E7 => 12,
            Coordinate::F7 => 13,
            Coordinate::G7 => 14,
            Coordinate::H7 => 15,
            Coordinate::A6 => 16,
            Coordinate::B6 => 17,
            Coordinate::C6 => 18,
            Coordinate::D6 => 19,
            Coordinate::E6 => 20,
            Coordinate::F6 => 21,
            Coordinate::G6 => 22,
            Coordinate::H6 => 23,
            Coordinate::A5 => 24,
            Coordinate::B5 => 25,
            Coordinate::C5 => 26,
            Coordinate::D5 => 27,
            Coordinate::E5 => 28,
            Coordinate::F5 => 29,
            Coordinate::G5 => 30,
            Coordinate::H5 => 31,
            Coordinate::A4 => 32,
            Coordinate::B4 => 33,
            Coordinate::C4 => 34,
            Coordinate::D4 => 35,
            Coordinate::E4 => 36,
            Coordinate::F4 => 37,
            Coordinate::G4 => 38,
            Coordinate::H4 => 39,
            Coordinate::A3 => 40,
            Coordinate::B3 => 41,
            Coordinate::C3 => 42,
            Coordinate::D3 => 43,
            Coordinate::E3 => 44,
            Coordinate::F3 => 45,
            Coordinate::G3 => 46,
            Coordinate::H3 => 47,
            Coordinate::A2 => 48,
            Coordinate::B2 => 49,
            Coordinate::C2 => 50,
            Coordinate::D2 => 51,
            Coordinate::E2 => 52,
            Coordinate::F2 => 53,
            Coordinate::G2 => 54,
            Coordinate::H2 => 55,
            Coordinate::A1 => 56,
            Coordinate::B1 => 57,
            Coordinate::C1 => 58,
            Coordinate::D1 => 59,
            Coordinate::E1 => 60,
            Coordinate::F1 => 61,
            Coordinate::G1 => 62,
            Coordinate::H1 => 63,
        }// }}}
    }
    pub fn from_index(index: usize) -> Self {
        match index {// {{{
            0 => Coordinate::A8,
            1 => Coordinate::B8,
            2 => Coordinate::C8,
            3 => Coordinate::D8,
            4 => Coordinate::E8,
            5 => Coordinate::F8,
            6 => Coordinate::G8,
            7 => Coordinate::H8,
            8 => Coordinate::A7,
            9 => Coordinate::B7,
            10 => Coordinate::C7,
            11 => Coordinate::D7,
            12 => Coordinate::E7,
            13 => Coordinate::F7,
            14 => Coordinate::G7,
            15 => Coordinate::H7,
            16 => Coordinate::A6,
            17 => Coordinate::B6,
            18 => Coordinate::C6,
            19 => Coordinate::D6,
            20 => Coordinate::E6,
            21 => Coordinate::F6,
            22 => Coordinate::G6,
            23 => Coordinate::H6,
            24 => Coordinate::A5,
            25 => Coordinate::B5,
            26 => Coordinate::C5,
            27 => Coordinate::D5,
            28 => Coordinate::E5,
            29 => Coordinate::F5,
            30 => Coordinate::G5,
            31 => Coordinate::H5,
            32 => Coordinate::A4,
            33 => Coordinate::B4,
            34 => Coordinate::C4,
            35 => Coordinate::D4,
            36 => Coordinate::E4,
            37 => Coordinate::F4,
            38 => Coordinate::G4,
            39 => Coordinate::H4,
            40 => Coordinate::A3,
            41 => Coordinate::B3,
            42 => Coordinate::C3,
            43 => Coordinate::D3,
            44 => Coordinate::E3,
            45 => Coordinate::F3,
            46 => Coordinate::G3,
            47 => Coordinate::H3,
            48 => Coordinate::A2,
            49 => Coordinate::B2,
            50 => Coordinate::C2,
            51 => Coordinate::D2,
            52 => Coordinate::E2,
            53 => Coordinate::F2,
            54 => Coordinate::G2,
            55 => Coordinate::H2,
            56 => Coordinate::A1,
            57 => Coordinate::B1,
            58 => Coordinate::C1,
            59 => Coordinate::D1,
            60 => Coordinate::E1,
            61 => Coordinate::F1,
            62 => Coordinate::G1,
            63 => Coordinate::H1,
            _  => panic!("Invalid index"),
        }// }}}
    }
}


#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Color {
    White,
    Black,
}
impl Color {
    pub fn from(turn: &str) -> Self {
        match turn {
            "w" => Color::White,
            "b" => Color::Black,
            _   => panic!("Invalid FEN (turn)"), // TODO: Handle error
        }
    }
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CastlingMove {
    king_move: BasicMove,
    rook_move: BasicMove,
}

impl CastlingMove {
     fn dbg_moves(castling_moves: &[CastlingMove]) {
        for moves in castling_moves.iter() {
            let start_indices: [usize; 2] = [moves.king_move, moves.rook_move]
                .iter()
                .map(|m| m.start_index)
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap();

            let between_indices = moves.get_squares().0;
            for i in 0..64 {
                if start_indices.contains(&i) {
                    print!("{} ", '0'.to_string().bright_yellow());
                } else if between_indices.contains(&i) {
                    print!("{} ", 'x'.to_string().blue())
                } else {
                    print!("{} ", '.'.to_string().black())
                }
                if i % 8 == 7 { println!() };
            }
            println!();
        }
    }

    fn from(king_move: &[usize; 2], rook_move: &[usize; 2]) -> Self {
        CastlingMove {
            king_move: BasicMove::from(king_move),
            rook_move: BasicMove::from(rook_move),
        }
    }

    /// Returns vec![empty indices], vec![king indices], king_index, rook_index
    pub fn get_squares(&self) -> (Vec<usize>, Vec<usize>, usize, usize) {
        if self == &CastlingMove::from(&[60, 62], &[63, 61]) {
             (vec![61, 62], vec![61, 62], 60, 63)
        } else if self == &CastlingMove::from(&[4, 6], &[7, 5]) {
             (vec![5, 6], vec![5, 6], 4, 7)
        } else if self == &CastlingMove::from(&[60, 58], &[56, 59]) {
             (vec![57, 58, 59], vec![58, 59], 60, 56)
        } else if self == &CastlingMove::from(&[4, 2], &[0, 3]) {
             (vec![1, 2, 3], vec![2, 3], 4, 0)
        } else {
            panic!("Invalid castling move");

        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PromotionMove {
    pawn_move: BasicMove,
    promotion_kind: PieceKind,
}

impl PromotionMove {
    fn from_move(pawn_move: BasicMove) -> Vec<Self> {
        vec![
            PromotionMove {
                pawn_move,
                promotion_kind: PieceKind::Rook,
            },
            PromotionMove {
                pawn_move,
                promotion_kind: PieceKind::Bishop,
            },
            PromotionMove {
                pawn_move,
                promotion_kind: PieceKind::Queen,
            },
            PromotionMove {
                pawn_move,
                promotion_kind: PieceKind::Knight,
            },
        ]
    }

    fn dbg_moves(promotion_moves: &[PromotionMove], board: &Board) {
        let mut start_indices: Vec<usize> = promotion_moves
            .iter()
            .map(|pm| pm.pawn_move.start_index)
            .collect();
        start_indices.dedup();

        let moves_list: Vec<[usize; 2]> = promotion_moves
            .iter()
            .map(|m| [m.pawn_move.start_index, m.pawn_move.target_index])
            .collect();
        for start_index in start_indices.into_iter() {
            for i in 0..64 {
                let piece = board.squares[i].display();
                if i == start_index {
                    print!("{} ", piece);
                } else if moves_list.contains(&[start_index, i]) {
                    print!("{} ", 'x'.to_string().bright_blue())
                } else {
                    print!("{} ", '.'.to_string().black())
                }
                if i % 8 == 7 { println!() };
            }
            println!();
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum CastlingRights {
    Neither,
    Kingside,
    Queenside,
    Both,
}
impl CastlingRights {
    pub fn from(king: bool, queen: bool) -> Self {
        match (king, queen) {
            (true, true) => CastlingRights::Both,
            (true, false) => CastlingRights::Kingside,
            (false, true) => CastlingRights::Queenside,
            (false, false) => CastlingRights::Neither,
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            CastlingRights::Neither   => "",
            CastlingRights::Kingside  => "k",
            CastlingRights::Queenside => "q",
            CastlingRights::Both      => "kq",
        }
    }
    pub fn gen_moves(&self, color: &Color) -> Result<HashSet<Move>, String> {
        // Magic values: king, rook
        let kingside = match color {
            Color::White => Move::CastlingMove(CastlingMove::from(
                &[60, 62],
                &[63, 61],
            )),
            Color::Black => Move::CastlingMove(CastlingMove::from(
                &[4, 6],
                &[7, 5],
            )),
            // can assume rook and king square have their respective
            // pieces as validated by castling rights, therefore do
            // not include 4, 7 in squares to check for emptiness
        };
        let queenside = match color {
            Color::White => Move::CastlingMove(CastlingMove::from(&[60,58], &[56,59])),
            Color::Black => Move::CastlingMove(CastlingMove::from(&[4, 2 ], &[0, 3 ])),
        };

        match self {
            CastlingRights::Neither   => Ok(HashSet::from([])),
            CastlingRights::Kingside  => Ok(HashSet::from([kingside])),
            CastlingRights::Queenside => Ok(HashSet::from([queenside])),
            CastlingRights::Both      => Ok(HashSet::from([kingside, queenside])),
        }
    }
    pub fn revoke(&mut self, right: CastlingRights) {
        match right {
            CastlingRights::Neither => return,
            CastlingRights::Both => { *self = CastlingRights::Neither; return },
            _ => (),
        }
        match self {
            CastlingRights::Neither => (),
            CastlingRights::Kingside => match right {
                CastlingRights::Kingside =>  *self = CastlingRights::Neither ,
                CastlingRights::Queenside => (),
                _ => (),
            },
            CastlingRights::Queenside => match right {
                CastlingRights::Kingside => (),
                CastlingRights::Queenside => *self = CastlingRights::Neither,
                _ => (),
            },
            CastlingRights::Both => match right {
                CastlingRights::Kingside => *self = CastlingRights::Queenside,
                CastlingRights::Queenside => *self = CastlingRights::Kingside,
                _ => (),
            },
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct CastlingState {
    white: CastlingRights,
    black: CastlingRights,
}
impl CastlingState {
    pub fn from(rights: &str) -> Self {
        if rights == "-" {
            return CastlingState {
                white: CastlingRights::Neither,
                black: CastlingRights::Neither,
            }
        }
        let white_k = rights.contains('K');
        let white_q = rights.contains('Q');
        let black_k = rights.contains('k');
        let black_q = rights.contains('q');

        CastlingState {
            white: CastlingRights::from(white_k, white_q),
            black: CastlingRights::from(black_k, black_q),
        }
    }
    pub fn to_str(&self) -> String {
        format!(
            "{}{}",
            self.white.to_str().to_ascii_uppercase(),
            self.black.to_str().to_ascii_lowercase().as_str()
        )
    }
    pub fn get_moves(&self, color: &Color) -> Result<HashSet<Move>, String> {
        match color {
            Color::White => self.white.gen_moves(color),
            Color::Black => self.black.gen_moves(color),
        }
    }
    pub fn revoke(&mut self, right: CastlingRights, color: &Color) {
        match color {
            Color::White => self.white.revoke(right),
            Color::Black => self.black.revoke(right),
        }
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl PieceKind {
    pub fn to_char(&self) -> char {
        match self {
            PieceKind::King   => 'k',
            PieceKind::Queen  => 'q',
            PieceKind::Rook   => 'r',
            PieceKind::Bishop => 'b',
            PieceKind::Knight => 'n',
            PieceKind::Pawn   => 'p',
        }
    }

    pub fn from(char: char) -> Self {
        match char {
            'k' => PieceKind::King,
            'q' => PieceKind::Queen,
            'r' => PieceKind::Rook,
            'b' => PieceKind::Bishop,
            'n' => PieceKind::Knight,
            'p' => PieceKind::Pawn,
            _   => panic!("Invalid piece: {}", char), // TODO: Handle error
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

impl Piece {
    pub fn to_char(&self) -> char {
        match self.color {
            Color::White => self.kind.to_char().to_ascii_uppercase(),
            Color::Black => self.kind.to_char().to_ascii_lowercase(),
        }
    }

    pub fn is_same_color(&self, piece: &Piece) -> bool {
        self.color == piece.color
    }

    pub fn into(self) -> Square {
        Square::Some(self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Square {
    Some(Piece),
    Empty,
}

impl Square {
    pub fn to_char(&self) -> char {
        match self {
            Square::Some(piece) => piece.to_char(),
            Square::Empty => '.',
        }
    }
    pub fn display(&self) -> ColoredString {
        let string = self.to_char().to_string();
        match self {
            Square::Some(piece) => match piece.kind {
                PieceKind::King => match piece.color {
                    Color::White => string.bright_yellow().bold(),
                    Color::Black => string.bright_blue().bold(),
                }
                _ => match piece.color {
                    Color::White => string.bright_white().bold(),
                    Color::Black => string.bright_black().bold(),
                },
            },
            Square::Empty => string.black(),
        }
    }
    pub fn piece(&self) -> Result<&Piece, String> {
        match self {
            Square::Some(piece) => Ok(piece),
            Square::Empty => Err("Square is empty".to_string())
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Square::Some(_) => false,
            Square::Empty => true,
        }
    }
    pub fn is_same_color(&self, square: Square) -> bool {
        if self.is_empty() || square.is_empty() {
            return false;
        }
        self.piece().unwrap().color == square.piece().unwrap().color
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BasicMove {
    pub start_index: usize,
    pub target_index: usize,
}

impl BasicMove {
    pub fn from(m: &[usize; 2]) -> Self {
        BasicMove {
            start_index: m[0],
            target_index: m[1],
        }
    }
    pub fn into(self) -> Move {
        Move::BasicMove(self)
    }
    pub fn dbg_moves(moves: &[BasicMove], board: &Board) {
        let mut start_indices: Vec<usize> = moves
            .iter()
            .map(|m| m.start_index)
            .collect();
        start_indices.dedup();

        let moves_list: Vec<[usize; 2]> = moves
            .iter()
            .map(|m| [ m.start_index, m.target_index ])
            .collect();

        for start_index in start_indices.into_iter() {
            for i in 0..64 {
                let piece = board.squares[i].display();
                if i == start_index {
                    print!("{} ", piece);
                } else if moves_list.contains(&[start_index, i]) {
                    print!("{} ", 'x'.to_string().bright_blue())
                } else {
                    print!("{} ", '.'.to_string().black())
                }
                if i % 8 == 7 { println!() };
            }
            println!();
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Move {
    BasicMove(BasicMove),
    CastlingMove(CastlingMove),
    PromotionMove(PromotionMove),
}

impl Move {
    pub fn to_string(&self) -> String {
        let start_index = match self {
            Move::BasicMove(bm) => bm.start_index,
            Move::CastlingMove(cm) => cm.king_move.start_index,
            Move::PromotionMove(pm) => pm.pawn_move.start_index,
        };
        let target_index = match self {
            Move::BasicMove(bm) => bm.target_index,
            Move::CastlingMove(cm) => cm.king_move.target_index,
            Move::PromotionMove(pm) => pm.pawn_move.target_index,
        };
        let start_index = Coordinate::from_index(start_index).to_string();
        let target_index = Coordinate::from_index(target_index).to_string();
        format!("{start_index}{target_index}")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum GameState {
    Playing,
    Draw,
    Victory(Color),
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BoardState {
    pub turn: Color,
    castling_state: CastlingState,
    pub ep_index: Option<usize>,
    pub halfmove_clock: u32,
    fullmove_count: u32,
    pub game_state: GameState,
    last_captured_square: Option<Square>,
    last_move: Option<Move>,
    last_ep_taken_index: Option<usize>,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Implementation {
    Clone,
    Unmove,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Board {
    pub squares: [Square; 64],
    pub state: BoardState,
    pub state_history: Vec<BoardState>,
    pub history: Vec<[Square; 64]>,
    pub implementation: Implementation,
}

impl Board {
    pub fn from(chars: &[char; 64]) -> Result<Self, String> {
        let mut squares = [Square::Empty; 64];
        for (i, piece_char) in chars.iter().enumerate() {
            let piece = match piece_char.to_ascii_lowercase() {
                'k' => Some(PieceKind::King),
                'q' => Some(PieceKind::Queen),
                'r' => Some(PieceKind::Rook),
                'b' => Some(PieceKind::Bishop),
                'n' => Some(PieceKind::Knight),
                'p' => Some(PieceKind::Pawn),
                ' ' => None,
                _ => None, // TODO: Handle error
            };
            squares[i] = match piece_char {
                'k' | 'q' | 'r' | 'b' | 'n' | 'p' => Piece {
                    kind: piece.unwrap(),
                    color: Color::Black
                }.into(),
                'K' | 'Q' | 'R' | 'B' | 'N' | 'P' => Piece {
                    kind: piece.unwrap(),
                    color: Color::White
                }.into(),
                ' ' => Square::Empty,
                _ => return Err(format!("Invalid char: `{}`", piece_char)),
            };
        }
        Ok(Board {
            squares,
            state: BoardState {
                turn: Color::White,
                castling_state: CastlingState {
                    white: CastlingRights::Both,
                    black: CastlingRights::Both,
                },
                ep_index: None,
                halfmove_clock: 0,
                fullmove_count: 1,
                game_state: GameState::Playing,
                last_captured_square: None,
                last_move: None,
                last_ep_taken_index: None,
            },
            state_history: vec![],
            history: vec![],
            implementation: Implementation::Clone,
        })
    }
    pub fn new() -> Self {
        Board::from(&[
            'r','n','b','q','k','b','n','r',
            'p','p','p','p','p','p','p','p',
            ' ',' ',' ',' ',' ',' ',' ',' ',
            ' ',' ',' ',' ',' ',' ',' ',' ',
            ' ',' ',' ',' ',' ',' ',' ',' ',
            ' ',' ',' ',' ',' ',' ',' ',' ',
            'P','P','P','P','P','P','P','P',
            'R','N','B','Q','K','B','N','R',
        ]).unwrap()
    }

    pub fn display(&self) {
        for (i, square) in self.squares.iter().enumerate() {
            print!("{} ", square.display());
            if i % 8 == 7 {
                for j in i - 7..=i {
                    print!(" {}", format!("{: >2}", j).black());
                }
                println!();
            }
        }
    }

    pub fn dbg_moves(
        &self,
        moves: &HashSet<Move>,
        shown_pieces: Vec<PieceKind>,
        show_castling: bool,
    ) -> Result<(), String> {
        BasicMove::dbg_moves(&moves
            .iter()
            .filter_map(|m| match m {
                Move::BasicMove(bm) => Some(*bm),
                _ => None
            })
            .filter(|m| {
                shown_pieces == vec![] ||
                shown_pieces.contains(&self.squares[m.start_index]
                    .piece()
                    .unwrap()
                    .kind
                )
            })
            .collect::<Vec<_>>(),
            self
        );
        if !show_castling { return Ok(()) }
        CastlingMove::dbg_moves(&moves
            .iter()
            .filter_map(|m| match m {
                Move::CastlingMove(cm) => Some(*cm), _ => None
            })
            .collect::<Vec<_>>()
        );
        PromotionMove::dbg_moves(&moves
            .iter()
            .filter_map(|m| match m{
                Move::PromotionMove(pm) => Some(*pm),
                _ => None
            })
            .collect::<Vec<_>>(),
            self
        );
        Ok(())
    }

    pub fn from_fen(fen: &str) -> Result<Self, String> {
        // TODO: Handle errors
        let mut squares: Vec<char> = vec![];
        let mut iter = fen.split(' ');
        let pieces            = iter.next().expect("Invalid FEN");
        let turn              = iter.next().expect("Invalid FEN");
        let castling_rights   = iter.next().expect("Invalid FEN");
        let ep_square         = iter.next().expect("Invalid FEN");
        let halfmove_clock    = iter.next().expect("Invalid FEN");
        let fullmove_count    = iter.next().expect("Invalid FEN");
        for row in pieces.split('/') {
            for c in row.chars() {
                match c {
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                        squares.append(
                            &mut vec![' '; c.to_digit(10).unwrap() as usize]
                        )
                    }
                    'k' | 'q' | 'r' | 'b' | 'n' | 'p' |
                    'K' | 'Q' | 'R' | 'B' | 'N' | 'P' => {
                        squares.append(&mut vec![c])
                    }
                    _ => return Err(format!("Invalid FEN (char): {}", fen)),
                }
            }
        }

        let array: [char; 64] = match squares.try_into() {
            Ok(some) => some,
            Err(_) => return Err(format!("Invalid FEN (len): {}", fen)),
        };

        let mut board = Board::from(&array)?;
        board.state.turn = Color::from(turn);
        board.state.castling_state = CastlingState::from(castling_rights);
        board.state.ep_index = Coordinate::from(ep_square)?
            .map(|coordinate| coordinate.into_index());
        board.state.halfmove_clock = halfmove_clock.parse::<u32>().unwrap();
        board.state.fullmove_count = fullmove_count.parse::<u32>().unwrap();
        Ok(board)
    }

    pub fn to_fen(&self) -> &str {
        let mut squares = [' '; 64];
        for (i, square) in self.squares.iter().enumerate() {
            squares[i] = square.to_char()
        }
        for char in squares.iter() {
            let _ = char; todo!()
        }
        todo!()
    }

    fn slider_gen_moves(&self, start_index: usize, moves: &mut HashSet<Move>) -> Result<(), String> {
        let square = self.squares[start_index];

        let mut start_dir_index = 0;
        let mut end_dir_index = 8;
        let mut branch_length = 8;

        let piece = square.piece()?;
        match piece.kind {
            PieceKind::Bishop => start_dir_index = 4,
            PieceKind::Rook => end_dir_index = 4,
            PieceKind::King => branch_length = 1,
            _ => (),
        }

        // for direction_index in start_dir_index..end_dir_index {
        for (direction_index, offset) in DIRECTION_OFFSETS
            .iter()
            .enumerate()
            .take(end_dir_index)
            .skip(start_dir_index) {
                for n in 0..branch_length.min(SQUARES_TO_EDGE[start_index][direction_index]) {
                    let target_index: usize = (start_index as i32
                    + (offset * (n + 1))) as usize;

                    let target_square = self.squares[target_index];

                    let target_piece = target_square.piece();
                    if let Ok(target_piece) = target_piece {
                        if target_piece.color == piece.color { break }
                    }

                    moves.insert(BasicMove {
                        start_index,
                        target_index,
                    }.into());

                    if !target_square.is_empty() { break }
                }
            }
        Ok(())
    }

    fn knight_gen_moves(&self, start_index: usize, moves: &mut HashSet<Move>) -> Result<(), String> {
        for offset in KNIGHT_XY_OFFSETS.iter() {
            let (start_x, start_y) = (start_index % 8, start_index / 8 % 8);
            let (target_x, target_y) = (
                start_x as i32 + offset[0],
                start_y as i32 + offset[1],
            );
            if (0..8).contains(&target_x)
            && (0..8).contains(&target_y) {
                let target_index = (start_index as i32 + offset[2]) as usize;
                let square = self.squares[start_index];
                let target_square = self.squares[target_index];

                if !square.is_same_color(target_square) {
                    moves.insert(BasicMove {
                        start_index,
                        target_index,
                    }.into());
                }
            }
        }
        Ok(())
    }

    fn pawn_gen_moves(&self, start_index: usize, moves: &mut HashSet<Move>) -> Result<(), String> {
        let square = self.squares[start_index];

        let rank = start_index / 8 % 8;
        let file = start_index % 8;

        let color = square.piece()?.color;

        let offsets = match color {
            Color::White => [-8, -16, -9, -7],
            Color::Black => [ 8,  16,  7,  9],
        };
        let invalid_ranks = match color {
            Color::White => [0, 1],
            Color::Black => [7, 6],
        };
        let starting_rank = match color {
            Color::White => 6,
            Color::Black => 1,
        };
        if rank == invalid_ranks[0] { return Ok(())  }
        if rank == invalid_ranks[1] {} // TODO: Promotion

        if rank != invalid_ranks[0] {
            let target_index: usize = (start_index as i32 + offsets[0]) as usize;
            let target_square = self.squares[target_index];
            if target_square.is_empty() {
                let mov = BasicMove {
                    start_index,
                    target_index,
                };
                if rank == invalid_ranks[1] {
                    for promotion_move in PromotionMove::from_move(mov).iter() {
                        moves.insert(Move::PromotionMove(*promotion_move));
                    }
                } else {
                    moves.insert(mov.into());
                }
                if rank != invalid_ranks[1]
                && rank == starting_rank{
                    let target_index: usize = (start_index as i32 + offsets[1]) as usize;
                    let target_square = self.squares[target_index];
                    if target_square.is_empty() {
                        moves.insert(BasicMove {
                            start_index,
                            target_index,
                        }.into());
                    }
                }
            }
        }
        if file != 0 {
            let target_index: usize = (start_index as i32 + offsets[2]) as usize;
            let target_square = self.squares[target_index];
            if (!target_square.is_same_color(square)
            && !target_square.is_empty())
            || Some(target_index) == self.state.ep_index
            {
                let mov = BasicMove {
                    start_index,
                    target_index,
                };
                if rank == invalid_ranks[1] {
                    for promotion_move in PromotionMove::from_move(mov).iter() {
                        moves.insert(Move::PromotionMove(*promotion_move));
                    }
                } else {
                    moves.insert(mov.into());
                }
            }
        }
        if file != 7 {
            let target_index: usize = (start_index as i32 + offsets[3]) as usize;
            let target_square = self.squares[target_index];
            if (!target_square.is_same_color(square)
            && !target_square.is_empty())
            || Some(target_index) == self.state.ep_index
            {
                let mov = BasicMove {
                    start_index,
                    target_index,
                };
                if rank == invalid_ranks[1] {
                    for promotion_move in PromotionMove::from_move(mov).iter() {
                        moves.insert(Move::PromotionMove(*promotion_move));
                    }
                } else {
                    moves.insert(mov.into());
                }
            }
        }
        Ok(())
    }

    fn gen_castling_moves(&self, moves: &mut HashSet<Move>) -> Result<(), String> {
        self.state.castling_state.get_moves(&self.state.turn)?
            .into_iter()
            .for_each(|m| {
                let mut empty_indices: Vec<usize> = vec![];
                let mut king_index: usize = 64;
                let mut rook_index: usize = 64;

                if let Move::BasicMove(_) = m {
                    panic!("`CastlingState.get_moves()` returned `BasicMoves`s");
                } else if let Move::CastlingMove(castling_move) = m {
                    (empty_indices, _, king_index, rook_index) = castling_move.get_squares();
                }
                for &empty_index in empty_indices.iter() {
                    match self.squares[empty_index] {
                        Square::Some(_) => return,
                        Square::Empty => (),
                    };
                }
                match self.squares[king_index] {
                    Square::Some(piece) => match piece.kind {
                        PieceKind::King => if piece.color != self.state.turn {
                            return;
                        },
                        _ => return,
                    },
                    Square::Empty => return,
                };
                match self.squares[rook_index] {
                    Square::Some(piece) => match piece.kind {
                        PieceKind::Rook => if piece.color != self.state.turn {
                            return;
                        },
                        _ => return,
                    },
                    Square::Empty => return,
                };
                moves.insert(m);
            });
        Ok(())
    }

    pub fn gen_pseudo_legal_moves(&self) -> Result<HashSet<Move>, String> {
        // TODO: Benchmark capacity past which performace gains diminish
        let mut moves: HashSet<Move> = HashSet::with_capacity(50);
        for start_index in 0..64 {
            let square = self.squares[start_index];
            if let Square::Some(piece) = square {
                if piece.color != self.state.turn { continue }
                match piece.kind {
                    PieceKind::Bishop | PieceKind::Rook | PieceKind::Queen | PieceKind::King
                                      => self.slider_gen_moves(start_index, &mut moves)?,
                    PieceKind::Knight => self.knight_gen_moves(start_index, &mut moves)?,
                    PieceKind::Pawn   => self.pawn_gen_moves(start_index, &mut moves)?,
                }
            }
        }
        self.gen_castling_moves(&mut moves)?;

        Ok(moves)
    }

    pub fn change_turn(&mut self) {
        self.state.turn = self.state.turn.opposite();
    }

    /// This function does not actually mutate self, as it calls `make_move`
    /// and `unmake_move` sequentially, without mutating anywhere else.
    pub fn gen_legal_moves(&mut self) -> Result<HashSet<Move>, String> {
        Ok(self.gen_pseudo_legal_moves()?.into_iter().filter(|m| {
            let mut king_indices: Vec<usize>;
            if let Move::CastlingMove(cm) = m {
                king_indices = cm.get_squares().1.to_vec();
                if self.attacked(self.king_index(self.state.turn).unwrap()).unwrap() {
                    return false;
                }
            } else {
                king_indices = vec![];
            }
            let opponent_responses: HashSet<Move>;
            let king_index: usize;

            // PERF: Cloning allows maintaining shared reference, but results
            // in a ~250% reduction in speed to play a full random game
            match self.implementation {
                Implementation::Clone => {
                    let mut board = self.clone();
                    board.make_move(m).unwrap();

                    opponent_responses = board.gen_pseudo_legal_moves().unwrap();
                    king_index = board.king_index(board.state.turn.opposite()).unwrap();

                },
                Implementation::Unmove => {
                    self.make_move(m).unwrap();

                    opponent_responses = self.gen_pseudo_legal_moves().unwrap();
                    king_index = self.king_index(self.state.turn.opposite()).unwrap();

                    self.unmake_move().unwrap();
                },
            }
            king_indices.push(king_index);

            for &response in opponent_responses.iter() {
                match response {
                    Move::BasicMove(response) => {
                        if king_indices.contains(&response.target_index) { return false; }
                    },
                    Move::PromotionMove(response) => {
                        if king_indices.contains(&response.pawn_move.target_index) { return false; }
                    },
                    Move::CastlingMove(_) => (),
                }
            }

            true
        }).collect())
    }

    pub fn make_move(&mut self, m: &Move) -> Result<(), String> {
        if self.implementation == Implementation::Unmove {
            self.state_history.push(self.state);
        }
        let color = self.state.turn;
        let mov: BasicMove = match m {
            Move::BasicMove(mov) => *mov,
            Move::CastlingMove(mov) => {
                self.state.castling_state.revoke(CastlingRights::Both, &color);
                mov.king_move
            },
            Move::PromotionMove(mov) => mov.pawn_move,
        };

        let ep_taken_index: Option<usize> = (|| {
            if let Some(index) = self.state.ep_index {
                if mov.target_index != index { return None }
                if let Square::Some(piece) = self.squares[mov.start_index] {
                    if piece.kind != PieceKind::Pawn { return None }
                    return Some((match color {
                        Color::White => mov.target_index + 8,
                        Color::Black => mov.target_index - 8,
                    }) as usize);
                }
            }
            None
        })();

        let moving_piece = self.squares[mov.start_index];
        let captured_piece = self.squares[mov.target_index];

        if self.implementation == Implementation::Unmove {
            self.state.last_ep_taken_index = ep_taken_index;
            self.state.last_move = Some(*m);
            self.state.last_captured_square = Some(self.squares[mov.target_index]);
        }

        self.state.ep_index = None;
        match captured_piece {
            Square::Some(piece) => {
                match piece.kind {
                    PieceKind::King => {
                        // println!("{:?} moved into check", color.opposite());
                        // NOTE: Could be an error in some cases, but necessary
                        // to allow to check if currently in check
                    },
                    PieceKind::Rook => {
                        match mov.target_index {
                            0 | 56 => self.state.castling_state.revoke(
                                CastlingRights::Queenside,
                                &color.opposite(),
                            ),
                            7 | 63 => self.state.castling_state.revoke(
                                CastlingRights::Kingside,
                                &color.opposite(),
                            ),
                            _  => (),
                        };
                    },
                    _ => (),
                };
                self.state.halfmove_clock = 0;
            },
            Square::Empty => {
                if ep_taken_index.is_none() {
                    self.state.halfmove_clock += 1
                }
            },
        };

        match moving_piece {
            Square::Some(piece) => match piece.kind {
                PieceKind::King => self.state.castling_state.revoke(
                    CastlingRights::Both,
                    &self.state.turn,
                ),
                PieceKind::Rook => match mov.start_index {
                    0 | 56 => self.state.castling_state
                        .revoke(CastlingRights::Queenside, &color),
                    7 | 63 => self.state.castling_state
                        .revoke(CastlingRights::Kingside,  &color),
                    _  => (),
                }
                PieceKind::Pawn => {
                    self.state.halfmove_clock = 0;
                    let offset: isize = mov.target_index as isize
                                      - mov.start_index as isize;
                    self.state.ep_index = match offset {
                        -16 => Some(mov.start_index - 8),
                        16  => Some(mov.start_index + 8),
                        _   => None,
                    };
                }
                _ => (),
            },
            Square::Empty => {
                return Err(format!("Attempt to move empty square {:?}", m))
            },

        };
        if self.state.halfmove_clock >= 50  {
            self.state.game_state = GameState::Draw
        }

        // if self.implementation == Implementation::Unmove {
        //     self.history.push(self.squares);
        // }
        match m {
            Move::BasicMove(m) => {
                if let Some(pawn_index) = ep_taken_index {
                    self.squares[pawn_index as usize] = Square::Empty;
                }
                self.squares[m.target_index] = self.squares[m.start_index];
                self.squares[m.start_index] = Square::Empty;
            },
            Move::CastlingMove(cm) => {
                self.squares[cm.king_move.target_index] = Square::Some(Piece{
                    kind: PieceKind::King,
                    color,
                });
                self.squares[cm.rook_move.target_index] = Square::Some(Piece{
                    kind: PieceKind::Rook,
                    color,
                });
                self.squares[cm.king_move.start_index] = Square::Empty;
                self.squares[cm.rook_move.start_index] = Square::Empty;
            }
            Move::PromotionMove(pm) => {
                self.squares[pm.pawn_move.target_index] = Square::Some(Piece { kind: pm.promotion_kind, color });
                self.squares[pm.pawn_move.start_index] = Square::Empty;
            }
        }

        if self.state.turn == Color::Black { self.state.fullmove_count += 1; }
        self.change_turn();

        Ok(())
    }

    pub fn unmake_move(&mut self) -> Result<(), String> {
        let last_captured_square = self.state.last_captured_square
            .expect("Attempt to undo from first move");
        let last_move = self.state.last_move
            .expect("Attempt to undo from first move");
        let last_ep_taken_index = self.state.last_ep_taken_index;

        self.state = self.state_history.pop()
            .expect("Attempt to undo from first move");

        // Color of the player who made the move being undone
        let color = self.state.turn;

        self.history.pop();
        match last_move {
            Move::BasicMove(mov) => {
                if let Some(index) = last_ep_taken_index {
                    self.squares[index] = Square::Some(Piece {
                        kind: PieceKind::Pawn,
                        color: self.state.turn.opposite(),
                    });
                }
                let last_moved_square = self.squares[mov.target_index];
                self.squares[mov.target_index] = last_captured_square;
                self.squares[mov.start_index] = last_moved_square;
            },
            Move::CastlingMove(cm) => {
                self.squares[cm.king_move.start_index] = Square::Some(Piece {
                    kind: PieceKind::King,
                    color,
                });
                self.squares[cm.rook_move.start_index] = Square::Some(Piece {
                    kind: PieceKind::Rook,
                    color,
                });
                self.squares[cm.king_move.target_index] = Square::Empty;
                self.squares[cm.rook_move.target_index] = Square::Empty;
            },
            Move::PromotionMove(pm) => {
                self.squares[pm.pawn_move.target_index] = last_captured_square;
                self.squares[pm.pawn_move.start_index] = Square::Some(Piece {
                    kind: PieceKind::Pawn,
                    color,
                });
            },
        }

        Ok(())
    }

    pub fn king_index(&self, color: Color) -> Result<usize, String> {
        match self.squares
            .iter()
            .position(|&square| square == Square::Some(Piece {
                kind: PieceKind::King, color,
            })) {
                Some(index) => Ok(index),
                None => {
                    println!("========== BOARD HISTORY ==========");
                    self.dbg_history();
                    self.display();
                    println!("===================================");
                    Err("Current player has no king!".to_string())
                },
            }
    }

    /// Only checks if the color whose turn it is is in check
    pub fn attacked(&self, index: usize) -> Result<bool, String> {
        let mut board = self.clone();
        board.change_turn();

        // NOTE: Shouldn't be necessary to update board state with play_move,
        // as none of the board state can make possible/impossible a capture
        // on the next turn: en passant cannot capture a king, 50-move rule is
        // irrelevant, as checkmate supersedes it, castling rights are
        // irrelevant as castling cannot capture a piece

        let moves = board.gen_pseudo_legal_moves()?;

        for mov in moves.iter() {
            match mov {
                Move::BasicMove(m) => {
                    if m.target_index == index { return Ok(true); }
                },
                Move::PromotionMove(pm) => {
                    if pm.pawn_move.target_index == index { return Ok(true); }
                }
                // Cannot capture by castling
                Move::CastlingMove(_) => (),
            }
        }

        Ok(false)
    }

    pub fn play_optional_move(
        &mut self,
        m: Option<&Move>,
    ) -> Result<(), String> {
        match m {
            Some(mov) => self.make_move(mov)?,
            None => {
                if self.attacked(self.king_index(self.state.turn)?)? {
                    self.state.game_state = GameState::Victory(
                        self.state.turn.opposite(),
                    );
                } else {
                    self.state.game_state = GameState::Draw;
                }
            }
        };
        Ok(())
    }

    pub fn dbg_play_move(&mut self, m: &[usize; 2]) -> Result<(), String> {
        self.make_move(&BasicMove::from(m).into())?;
        Ok(())
    }

    pub fn dbg_no_castle(&mut self) {
        self.state.castling_state.revoke(CastlingRights::Both, &Color::White);
        self.state.castling_state.revoke(CastlingRights::Both, &Color::Black);
    }

    pub fn dbg_move_count(&mut self) -> Result<u32, String> {
        Ok(self.gen_legal_moves()?.len() as u32)
    }

    pub fn dbg_depth_num_positions(&mut self, depth: i32) -> Result<u32, String> {
        if depth <= 0 { return Ok(1) }
        let moves = self.gen_legal_moves()?;
        let mut num_positions: u32 = 0;

        match self.implementation {
            Implementation::Clone => {
                for mov in moves.iter() {
                    let mut board = self.clone();
                    board.make_move(mov)?;
                    num_positions += board.dbg_depth_num_positions(depth - 1)?;
                }
            },
            Implementation::Unmove => {
                for mov in moves.iter() {
                    self.make_move(mov)?;
                    num_positions += self.dbg_depth_num_positions(depth - 1)?;
                    self.unmake_move()?;
                }
            },
        }

        Ok(num_positions)
    }

    pub fn perft(&mut self, depth: i32) -> Result<u32, String> {
        if depth <= 0 {
            return Ok(0);
        }
        let moves = self.gen_legal_moves()?;

        let mut total_positions: u32 = 0;

        for mov in moves.iter() {
            let num_moves = match self.implementation {
                Implementation::Clone => {
                    let mut board = self.clone();
                    board.make_move(mov)?;
                    board.dbg_depth_num_positions(depth - 1)?

                },
                Implementation::Unmove => {
                    self.make_move(mov)?;
                    let n = self.dbg_depth_num_positions(depth - 1)?;
                    self.unmake_move()?;
                    n
                },
            };
            total_positions += num_moves;
            println!("{}: {}", mov.to_string(), num_moves);
        }
        println!("Total Positions: {}", total_positions);
        Ok(total_positions)
    }

    pub fn dbg_history(&self) {
        for board in self.history.iter() {
            for (i, square) in board.iter().enumerate() {
                print!("{} ", square.display());
                if i % 8 == 7 {
                    for j in i - 7..=i {
                        print!(" {}", format!("{: >2}", j).black());
                    }
                    println!();
                }
            }
            println!();
        }
    }
    pub fn play_random_game(
        &mut self,
        move_limit: u32,
    ) -> Result<GameState, String> {
        use rand::{seq::SliceRandom, thread_rng};
        for _ in 0..move_limit {
            let moves: Vec<Move> = self.gen_legal_moves()?.into_iter().collect();

            let mut rng = thread_rng();
            let mov = moves.choose(&mut rng);
            self.play_optional_move(mov).unwrap();
            if self.state.game_state != GameState::Playing { break }
        }
        Ok(self.state.game_state)
    }

    pub fn dbg_gen_moves(&mut self) -> Result<(), String> {
        let moves = self.gen_legal_moves()?;
        self.dbg_moves(&moves, vec![], true)?;
        Ok(())
    }
}
