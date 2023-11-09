#![allow(dead_code)]

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
    pub fn from(coordinate: &str) -> Self {
        match coordinate {// {{{
            "a1" => Coordinate::A1,
            "a2" => Coordinate::A2,
            "a3" => Coordinate::A3,
            "a4" => Coordinate::A4,
            "a5" => Coordinate::A5,
            "a6" => Coordinate::A6,
            "a7" => Coordinate::A7,
            "a8" => Coordinate::A8,
            "b1" => Coordinate::B1,
            "b2" => Coordinate::B2,
            "b3" => Coordinate::B3,
            "b4" => Coordinate::B4,
            "b5" => Coordinate::B5,
            "b6" => Coordinate::B6,
            "b7" => Coordinate::B7,
            "b8" => Coordinate::B8,
            "c1" => Coordinate::C1,
            "c2" => Coordinate::C2,
            "c3" => Coordinate::C3,
            "c4" => Coordinate::C4,
            "c5" => Coordinate::C5,
            "c6" => Coordinate::C6,
            "c7" => Coordinate::C7,
            "c8" => Coordinate::C8,
            "d1" => Coordinate::D1,
            "d2" => Coordinate::D2,
            "d3" => Coordinate::D3,
            "d4" => Coordinate::D4,
            "d5" => Coordinate::D5,
            "d6" => Coordinate::D6,
            "d7" => Coordinate::D7,
            "d8" => Coordinate::D8,
            "e1" => Coordinate::E1,
            "e2" => Coordinate::E2,
            "e3" => Coordinate::E3,
            "e4" => Coordinate::E4,
            "e5" => Coordinate::E5,
            "e6" => Coordinate::E6,
            "e7" => Coordinate::E7,
            "e8" => Coordinate::E8,
            "f1" => Coordinate::F1,
            "f2" => Coordinate::F2,
            "f3" => Coordinate::F3,
            "f4" => Coordinate::F4,
            "f5" => Coordinate::F5,
            "f6" => Coordinate::F6,
            "f7" => Coordinate::F7,
            "f8" => Coordinate::F8,
            "g1" => Coordinate::G1,
            "g2" => Coordinate::G2,
            "g3" => Coordinate::G3,
            "g4" => Coordinate::G4,
            "g5" => Coordinate::G5,
            "g6" => Coordinate::G6,
            "g7" => Coordinate::G7,
            "g8" => Coordinate::G8,
            "h1" => Coordinate::H1,
            "h2" => Coordinate::H2,
            "h3" => Coordinate::H3,
            "h4" => Coordinate::H4,
            "h5" => Coordinate::H5,
            "h6" => Coordinate::H6,
            "h7" => Coordinate::H7,
            "h8" => Coordinate::H8,
            _    => panic!("Invalid coordinate {}", coordinate)
        }// }}}
    }
}


#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Color {
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
}

type CastlingMove = [BasicMove; 2];

trait MoveDebug {
     fn debug_moves(castling_moves: &Vec<CastlingMove>);
}

impl MoveDebug for CastlingMove {
     fn debug_moves(castling_moves: &Vec<CastlingMove>) {
        for moves in castling_moves.iter() {
            let start_indices: [usize; 2] = moves
                .iter()
                .map(|m| m.start_index)
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap();

            let target_indices: [usize; 2] = moves
                .iter()
                .map(|m| m.target_index)
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap();

            for i in 0..64 {
                if start_indices.contains(&i) {
                    print!("0 ");
                } else if target_indices.contains(&i) {
                    print!("x ")
                } else {
                    print!(". ")
                }
                if i % 8 == 7 { print!("\n") };
            }
            println!("");
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum CastlingRights {
    Neither,
    Kingside,
    Queenside,
    Both,
}
impl CastlingRights {
    pub fn from(rights: &str) -> Self {
        match rights {
            ""   => CastlingRights::Neither,
            "k"  => CastlingRights::Kingside,
            "q"  => CastlingRights::Queenside,
            "kq" => CastlingRights::Both,
            _    => CastlingRights::Neither, // TODO: Handle error
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
    pub fn gen_moves(&self, square: &Square) -> Result<Vec<CastlingMove>, String> {
        // Magic values: king, rook
        let piece = square.piece()?;
        let color = piece.color;
        match piece.kind {
            PieceKind::King => (),
            _ => return Err(format!("Invalid Square: square {:?} has no color", &square)),
        }
        let kingside: CastlingMove = match color {
            Color::White => [
                BasicMove::from(&[60, 62], PieceKind::King),
                BasicMove::from(&[63, 61], PieceKind::Rook),
            ],
            Color::Black => [
                BasicMove::from(&[4, 6], PieceKind::King),
                BasicMove::from(&[7, 5], PieceKind::Rook),
            ],
        };
        let queenside: CastlingMove = match color {
            Color::White => [
                    BasicMove::from(&[60, 58], PieceKind::King),
                    BasicMove::from(&[56, 59], PieceKind::Rook),
            ],
            Color::Black => [
                    BasicMove::from(&[4, 2], PieceKind::King),
                    BasicMove::from(&[0, 3], PieceKind::Rook),
            ],
        };

        match self {
            CastlingRights::Neither   => Ok(vec![]),
            CastlingRights::Kingside  => Ok(vec![kingside]),
            CastlingRights::Queenside => Ok(vec![queenside]),
            CastlingRights::Both      => Ok(vec![kingside, queenside]),
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
        CastlingState {
            white: CastlingRights::from(&rights[0..2].to_ascii_lowercase()),
            black: CastlingRights::from(&rights[2..4].to_ascii_lowercase()),
        }
    }
    pub fn to_str(&self) -> String {
        return format!(
            "{}{}",
            self.white.to_str().to_ascii_uppercase(),
            self.black.to_str().to_ascii_lowercase().as_str()
        );
    }
    pub fn get_moves(&self, square: &Square) -> Result<Vec<Move>, String> {
        Ok(match square.piece()?.color {
            Color::White => self.white.gen_moves(&square),
            Color::Black => self.black.gen_moves(&square),
        }?.iter().map(|m| Move::CastlingMove(*m)).collect())
    }
}
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
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
struct Piece {
    kind: PieceKind,
    color: Color,
}

impl Piece {
    pub fn to_char(&self) -> char {
        match self.color {
            Color::White => self.kind.to_char().to_ascii_uppercase(),
            Color::Black => self.kind.to_char().to_ascii_lowercase(),
        }
    }

    pub fn is_same_color(&self, piece: &Piece) -> bool {
        match (self.color, piece.color,) {
            (Color::White, Color::White) |
            (Color::Black, Color::Black) => true,
            _ => false,
        }
    }

    pub fn into(self) -> Square {
        Square::Some(self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Square {
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
        } else {
            return self.piece().unwrap().is_same_color(square.piece().unwrap());
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BasicMove {
    pub start_index: usize,
    pub target_index: usize,
    pub piece: PieceKind,
}

impl BasicMove {
    pub fn from(m: &[usize; 2], piece: PieceKind) -> Self {
        BasicMove {
            start_index: m[0],
            target_index: m[1],
            piece,
        }
    }
    pub fn into(self) -> Move {
        Move::BasicMove(self)
    }
    pub fn debug_moves(moves: &Vec<BasicMove>) {
        let mut start_squares: Vec<(usize, char)> = moves
            .iter()
            .map(|m| (m.start_index, m.piece.to_char()))
            .collect::<Vec<_>>();
        start_squares.dedup();

        let moves_list: Vec<[usize; 2]> = moves
            .iter()
            .map(|m| [ m.start_index, m.target_index ])
            .collect();

        for (start_square, piece) in start_squares.iter() {
            for i in 0..64 {
                if i == *start_square {
                    print!("{} ", piece);
                } else if moves_list.contains(&[*start_square, i]) {
                    print!("x ")
                } else {
                    print!(". ")
                }
                if i % 8 == 7 { print!("\n") };
            }
            println!("");
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Move {
    BasicMove(BasicMove),
    CastlingMove(CastlingMove),
}

impl Move {
    pub fn debug_moves(moves: &Vec<Move>, ignored_pieces: Vec<PieceKind>, show_castling: bool) {
        BasicMove::debug_moves(&moves
            .iter()
            .filter_map(|m| match m { Move::BasicMove(bm) => Some(*bm), _ => None })
            .filter(|m| !ignored_pieces.contains(&m.piece))
            .collect()
        );
        if !show_castling { return }
        CastlingMove::debug_moves(&moves
            .iter()
            .filter_map(|m| match m { Move::CastlingMove(cm) => Some(*cm), _ => None })
            .collect()
        );
    }
}


#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Board {
    squares: [Square; 64],
    turn: Color,
    castling_state: CastlingState,
    pub en_passant_index: Option<usize>,
    halfmove_clock: u32,
    fullmove_count: u32,
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
            turn: Color::White,
            castling_state: CastlingState {
                white: CastlingRights::Both,
                black: CastlingRights::Both,
            },
            en_passant_index: None,
            halfmove_clock: 0,
            fullmove_count: 1,
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
            print!("{} ", square.to_char());
            if i % 8 == 7 {
                for j in i - 7..=i {
                    print!(" {}", format!("{: >2}", j));
                }
                print!("\n");
            }
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self, String> {
        // TODO: Handle errors
        let mut squares: Vec<char> = vec![];
        let mut iter = fen.split(' ');
        let pieces            = iter.next().unwrap();
        let turn              = iter.next().unwrap();
        let castling_rights   = iter.next().unwrap();
        let en_passant_square = iter.next().unwrap();
        let halfmove_clock    = iter.next().unwrap();
        let fullmove_count    = iter.next().unwrap();
        for row in pieces.split('/') {
            for c in row.chars() {
                match c {
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                        squares.append(&mut vec![' '; c.to_digit(10).unwrap() as usize])
                    }
                    'k' | 'q' | 'r' | 'b' | 'n' | 'p' | 'K' | 'Q' | 'R' | 'B' | 'N' | 'P' => {
                        squares.append(&mut vec![c])
                    }
                    _ => panic!("Invalid FEN (char): {}", fen),
                }
            }
        }

        let array: [char; 64] = match squares.try_into() {
            Ok(some) => some,
            Err(_) => panic!("Invalid FEN (len): {}", fen),
        };

        let mut board = Board::from(&array)?;
        board.turn = Color::from(turn);
        board.castling_state = CastlingState::from(castling_rights);
        board.en_passant_index = match en_passant_square.parse::<usize>() {
            Ok(index) => Some(index),
            Err(_) => None,
        };
        board.halfmove_clock = halfmove_clock.parse::<u32>().unwrap();
        board.fullmove_count = fullmove_count.parse::<u32>().unwrap();
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

    fn slider_gen_moves(&self, start_index: usize) -> Result<Vec<Move>, String> {
        let mut moves: Vec<Move> = vec![];
        let square = self.squares[start_index as usize];

        let mut start_dir_index = 0;
        let mut end_dir_index = 8;
        let mut branch_length = 8;

        let piece = square.piece()?.kind;
        match piece {
            PieceKind::Bishop => start_dir_index = 4,
            PieceKind::Rook => end_dir_index = 4,
            PieceKind::King => branch_length = 1,
            _ => (),
        }

        for direction_index in start_dir_index..end_dir_index {
            for n in 0..branch_length.min(SQUARES_TO_EDGE[start_index][direction_index]) {
                let target_index: usize = (start_index as i32
                + DIRECTION_OFFSETS[direction_index] * (n + 1)) as usize;

                let target_square = self.squares[target_index];

                if square.is_same_color(target_square) { break }

                moves.push(BasicMove {
                    start_index,
                    target_index,
                    piece,
                }.into());

                if target_square.is_empty() { break }
            }
        }
        Ok(moves)
    }

    fn knight_gen_moves(&self, start_index: usize) -> Result<Vec<Move>, String> {
        let mut moves: Vec<Move> = vec![];
        for offset in KNIGHT_XY_OFFSETS.iter() {
            let (start_x, start_y) = (start_index % 8, start_index / 8 % 8);
            let (target_x, target_y) = (
                start_x as i32 + offset[0],
                start_y as i32 + offset[1],
            );
            if 0 <= target_x && target_x < 8
            && 0 <= target_y && target_y < 8 {
                let target_index = (start_index as i32 + offset[2]) as usize;
                let square = self.squares[start_index];
                let target_square = self.squares[target_index];

                if !square.is_same_color(target_square) {
                    moves.push(BasicMove {
                        start_index,
                        target_index,
                        piece: PieceKind::Knight,
                    }.into());
                }
            }
        }
        Ok(moves)
    }

    fn pawn_gen_moves(&self, start_index: usize) -> Result<Vec<Move>, String> {
        let mut moves: Vec<Move> = vec![];
        let square = self.squares[start_index as usize];

        let rank = start_index / 8 % 8;
        let file = start_index % 8;

        let color = square.piece()?.color;

        let offsets = match color {
            Color::White => [-8, -16, -9, -7],
            Color::Black => [ 8,  16,  7,  9],
        };
        let invalid_ranks = match color {
            Color::White => [1, 0],
            Color::Black => [6, 7],
        };
        let starting_rank = match color {
            Color::White => 6,
            Color::Black => 1,
        };
        if rank != invalid_ranks[0] {
            let target_index: usize = (start_index as i32 + offsets[0]) as usize;
            let target_square = self.squares[target_index];
            if target_square.is_empty() {
                moves.push(BasicMove {
                    start_index,
                    target_index,
                    piece: PieceKind::Pawn,
                }.into());
                if rank != invalid_ranks[1]
                && rank == starting_rank{
                    let target_index: usize = (start_index as i32 + offsets[1]) as usize;
                    let target_square = self.squares[target_index];
                    if target_square.is_empty() {
                        moves.push(BasicMove {
                            start_index,
                            target_index,
                            piece: PieceKind::Pawn
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
            || Some(target_index) == self.en_passant_index {
                moves.push(BasicMove {
                    start_index,
                    target_index,
                    piece: PieceKind::Pawn
                }.into());
            }
        }
        if file != 7 {
            let target_index: usize = (start_index as i32 + offsets[3]) as usize;
            let target_square = self.squares[target_index];
            if !target_square.is_same_color(square)
            && !target_square.is_empty() {
                moves.push(BasicMove {
                    start_index,
                    target_index,
                    piece: PieceKind::Pawn
                }.into());
            }
        }
        Ok(moves)
    }

    pub fn generate_moves(&self) -> Result<Vec<Move>, String> {
        let mut moves: Vec<Move> = vec![];
        for start_index in 0..64 {
            let square = self.squares[start_index];
            match square {
                Square::Some(piece) => match piece.kind {
                    PieceKind::Bishop | PieceKind::Rook | PieceKind::Queen
                                  => moves.append(&mut self.slider_gen_moves(start_index)?),
                    PieceKind::Knight => moves.append(&mut self.knight_gen_moves(start_index)?),
                    PieceKind::Pawn   => moves.append(&mut self.pawn_gen_moves(start_index)?),
                    PieceKind::King   => {
                        moves.append(&mut self.slider_gen_moves(start_index)?);
                        moves.append(&mut self.castling_state.get_moves(&square)?);
                    }
                },
                _ => (),
            }
        }
        Ok(moves)
    }
}
