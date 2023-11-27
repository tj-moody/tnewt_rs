use crate::{board, color::Color};
use colored::{ColoredString, Colorize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl Kind {
    #[must_use]
    #[rustfmt::skip]
    pub fn to_char(&self) -> char {
        match self {
            Kind::King   => 'k',
            Kind::Queen  => 'q',
            Kind::Rook   => 'r',
            Kind::Bishop => 'b',
            Kind::Knight => 'n',
            Kind::Pawn   => 'p',
        }
    }

    /// Generate the appropriate piece Kind from `char`.
    ///
    /// # Errors
    ///
    /// This function will return an error if `char` is not one of 'k', 'q', 'r', 'b', 'n', or 'p'.
    pub fn from(char: char) -> Result<Self, board::Error> {
        match char {
            'k' => Ok(Kind::King),
            'q' => Ok(Kind::Queen),
            'r' => Ok(Kind::Rook),
            'b' => Ok(Kind::Bishop),
            'n' => Ok(Kind::Knight),
            'p' => Ok(Kind::Pawn),
            _ => Err(board::Error::InvalidPieceChar(char)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Piece {
    pub kind: Kind,
    pub color: Color,
}

pub type Square = Option<Piece>;

impl Piece {
    #[must_use]
    pub fn square_to_char(square: &Square) -> char {
        match square {
            Some(piece) => match piece.color {
                Color::White => piece.kind.to_char().to_ascii_uppercase(),
                Color::Black => piece.kind.to_char().to_ascii_lowercase(),
            },
            None => '.',
        }
    }
    #[must_use]
    pub fn display_square(square: &Square) -> ColoredString {
        let string = Piece::square_to_char(square).to_string();
        match square {
            Some(piece) => match piece.kind {
                Kind::King => match piece.color {
                    Color::White => string.bright_yellow().bold(),
                    Color::Black => string.bright_blue().bold(),
                },
                _ => match piece.color {
                    Color::White => string.bright_white().bold(),
                    Color::Black => string.bright_black().bold(),
                },
            },
            None => string.black(),
        }
    }

    #[must_use]
    pub fn is_same_color(square1: Square, square2: Square) -> bool {
        match square1 {
            None => false,
            Some(piece1) => match square2 {
                Some(piece2) => piece1.color == piece2.color,
                None => false,
            },
        }
    }

    pub fn get_piece(square: Square) -> Result<Piece, board::Error> {
        match square {
            Some(piece) => Ok(piece),
            None => Err(board::Error::MoveEmptySquare),
        }
    }
}
