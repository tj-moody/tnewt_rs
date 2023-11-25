use colored::Colorize;

use crate::board;
use crate::coordinate::Coordinate;
use crate::piece::{Piece, PieceKind};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Move {
    pub start_index: usize,
    pub target_index: usize,
    pub promotion_kind: Option<PieceKind>,
}

impl From<[usize; 2]> for Move {
    fn from(indices: [usize; 2]) -> Self {
        Move {
            start_index: indices[0],
            target_index: indices[0],
            promotion_kind: None,
        }
    }
}

impl Move {
    pub fn new(start_index: usize, target_index: usize) -> Self {
        Move {
            start_index,
            target_index,
            promotion_kind: None,
        }
    }

    pub fn set_promotion_kind(mut self, promotion: PieceKind) -> Self {
        self.promotion_kind = Some(promotion);
        self
    }

    pub fn indices(&self) -> (usize, usize) {
        (self.start_index, self.target_index)
    }

    pub fn dbg_moves(moves: &[Move], squares: &[Option<Piece>; 64]) {
        let mut start_indices: Vec<usize> = moves.iter().map(|m| m.start_index).collect();
        start_indices.dedup();

        let moves_list: Vec<[usize; 2]> = moves
            .iter()
            .map(|m| [m.start_index, m.target_index])
            .collect();

        for start_index in start_indices {
            for (i, square) in squares.iter().enumerate() {
                let piece = Piece::display_square(square);
                if i == start_index {
                    print!("{piece} ");
                } else if moves_list.contains(&[start_index, i]) {
                    print!("{} ", 'x'.to_string().bright_blue());
                } else {
                    print!("{} ", '.'.to_string().black());
                }
                if i % 8 == 7 {
                    println!()
                };
            }
            println!();
        }
    }

    pub fn promotion_moves(&self) -> Vec<Move> {
        vec![
            Move {
                promotion_kind: Some(PieceKind::Rook),
                ..*self
            },
            Move {
                promotion_kind: Some(PieceKind::Bishop),
                ..*self
            },
            Move {
                promotion_kind: Some(PieceKind::Knight),
                ..*self
            },
            Move {
                promotion_kind: Some(PieceKind::Queen),
                ..*self
            },
        ]
    }

    pub fn is_castling(&self, squares: &[Option<Piece>; 64]) -> Result<bool, board::Error> {
        if let Some(piece) = squares[self.start_index] {
            if piece.kind != PieceKind::King {
                return Ok(false);
            }
            let offset = self.target_index as i32 - self.start_index as i32;
            if offset != 2 && offset != -2 {
                return Ok(false);
            }
            Ok(true)
        } else {
            Err(board::Error::MoveEmptySquare)
        }
    }

    /// Returns which castling move this legal move is, if any, encoded by the target index.
    ///
    /// # Errors
    ///
    /// This function will return an error if the move originates from an empty square.
    pub fn castling_move(
        &self,
        squares: &[Option<Piece>; 64],
    ) -> Result<Option<usize>, board::Error> {
        let start_square = squares[self.start_index];
        if let Some(piece) = start_square {
            if piece.kind != PieceKind::King {
                return Ok(None);
            }
            let offset = self.target_index as i32 - self.start_index as i32;
            if offset != 2 && offset != -2 {
                return Ok(None);
            }
            if self.start_index != 4 && self.start_index != 60 {
                return Ok(None);
            }
            Ok(Some(self.target_index))
        } else {
            return Err(board::Error::MoveEmptySquare);
        }
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start_index = Coordinate::from_index(self.start_index).to_string();
        let target_index = Coordinate::from_index(self.target_index).to_string();
        let promotion_kind = match self.promotion_kind {
            Some(kind) => match kind {
                PieceKind::King => "E",
                PieceKind::Queen => "q",
                PieceKind::Rook => "r",
                PieceKind::Bishop => "b",
                PieceKind::Knight => "n",
                PieceKind::Pawn => "E",
            },
            None => "",
        };
        write!(f, "{start_index}{target_index}{promotion_kind}")
    }
}
