use crate::color::*;
use color_eyre::eyre::Result;
use colored::{Colorize, ColoredString};

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
