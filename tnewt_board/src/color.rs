use crate::board;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(C)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Generate color from fen representation.
    ///
    /// # Errors
    ///
    /// This function will return an error if `turn` is not "w" or "b".
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
