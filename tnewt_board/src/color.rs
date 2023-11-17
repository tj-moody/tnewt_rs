#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Color {
    White,
    Black,
}
impl Color {
    #[must_use]
    pub fn from(turn: &str) -> Self {
        match turn {
            "w" => Color::White,
            "b" => Color::Black,
            _   => panic!("Invalid FEN (turn)"), // TODO: Handle error
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

