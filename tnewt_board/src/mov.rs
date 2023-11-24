use colored::Colorize;

use crate::coordinate::Coordinate;
use crate::piece::{Piece, PieceKind};

/*
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CastlingMove {
    pub king_move: BasicMove,
    pub rook_move: BasicMove,
}

impl CastlingMove {
    pub fn dbg_moves(castling_moves: &[CastlingMove]) {
        for moves in castling_moves.iter() {
            let start_indices: [usize; 2] =
                [moves.king_move.start_index, moves.rook_move.start_index];

            let between_indices = moves.get_squares().0;
            for i in 0..64 {
                if start_indices.contains(&i) {
                    print!("{} ", '0'.to_string().bright_yellow());
                } else if between_indices.contains(&i) {
                    print!("{} ", 'x'.to_string().blue());
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

    #[must_use]
    pub fn from(king_move: &[usize; 2], rook_move: &[usize; 2]) -> Self {
        CastlingMove {
            king_move: BasicMove::from(king_move),
            rook_move: BasicMove::from(rook_move),
        }
    }

    /// Returns vec![empty indices], vec![king indices], king_index, rook_index
    #[must_use]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PromotionMove {
    pub pawn_move: BasicMove,
    pub promotion_kind: PieceKind,
}

impl PromotionMove {
    #[must_use]
    pub fn from_move(pawn_move: BasicMove) -> Vec<Self> {
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

    pub fn dbg_moves(promotion_moves: &[PromotionMove], squares: &[Option<Piece>; 64]) {
        let mut start_indices: Vec<usize> = promotion_moves
            .iter()
            .map(|pm| pm.pawn_move.start_index)
            .collect();
        start_indices.dedup();

        let moves_list: Vec<[usize; 2]> = promotion_moves
            .iter()
            .map(|m| [m.pawn_move.start_index, m.pawn_move.target_index])
            .collect();
        for start_index in start_indices {
            for (i, square) in squares.iter().enumerate() {
                let piece = Piece::display_square(square);
                if i == start_index {
                    print!("{} ", piece);
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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    #[must_use]
    pub fn into(self) -> Move {
        Move::BasicMove(self)
    }
    pub fn dbg_moves(moves: &[BasicMove], squares: &[Option<Piece>; 64]) {
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
}
*/

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
