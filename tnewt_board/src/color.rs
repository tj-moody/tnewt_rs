use crate::board;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn from(turn: &str) -> Result<Self, board::Error> {
        match turn {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => Err(board::Error::InvalidColorStr(turn.to_string())),
        }
    }
    #[must_use]
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
