use crate::board;
use crate::mov::Move;
use crate::color::Color;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Rights {
    Neither,
    Kingside,
    Queenside,
    Both,
}
impl Rights {
    #[must_use]
    pub fn from(king: bool, queen: bool) -> Self {
        match (king, queen) {
            (true, true) => Rights::Both,
            (true, false) => Rights::Kingside,
            (false, true) => Rights::Queenside,
            (false, false) => Rights::Neither,
        }
    }

    #[must_use]
    pub fn to_str(&self) -> &str {
        match self {
            Rights::Neither   => "",
            Rights::Kingside  => "k",
            Rights::Queenside => "q",
            Rights::Both      => "kq",
        }
    }
    pub fn gen_moves(&self, color: &Color) -> Vec<Move> {
        use Rights as R;
        if *self == R::Neither { return vec![]; }

        let kingside = match color {
            Color::White => Move::new(60, 62),
            Color::Black => Move::new(4, 6),
        };
        if *self == R::Kingside { return vec![kingside]; }

        let queenside = match color {
            Color::White => Move::new(60,58),
            Color::Black => Move::new(4, 2 ),
        };
        if *self == R::Queenside { return vec![queenside] }

        vec![kingside, queenside]
    }

    pub fn revoke(&mut self, right: Rights) {
        use Rights as R;
        match right {
            R::Neither => return,
            R::Both => { *self = R::Neither; return },
            _ => (),
        }
        match self {
            R::Neither => (),
            R::Kingside => if right == R::Kingside {
                *self = R::Neither;
            },
            R::Queenside => if right == R::Queenside {
                *self = R::Neither;
            },
            R::Both => match right {
                R::Kingside => *self = R::Queenside,
                R::Queenside => *self = R::Kingside,
                _ => (),
            },
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct State {
    pub white: Rights,
    pub black: Rights,
}
impl State {
    #[must_use]
    pub fn from(rights: &str) -> Self {
        if rights == "-" {
            return State {
                white: Rights::Neither,
                black: Rights::Neither,
            }
        }
        let white_k = rights.contains('K');
        let white_q = rights.contains('Q');
        let black_k = rights.contains('k');
        let black_q = rights.contains('q');

        State {
            white: Rights::from(white_k, white_q),
            black: Rights::from(black_k, black_q),
        }
    }
    #[must_use]
    pub fn to_str(&self) -> String {
        format!(
            "{}{}",
            self.white.to_str().to_ascii_uppercase(),
            self.black.to_str().to_ascii_lowercase().as_str()
        )
    }
    pub fn get_moves(&self, color: &Color) -> Vec<Move> {
        match color {
            Color::White => self.white.gen_moves(color),
            Color::Black => self.black.gen_moves(color),
        }
    }
    pub fn revoke(&mut self, right: Rights, color: &Color) {
        match color {
            Color::White => self.white.revoke(right),
            Color::Black => self.black.revoke(right),
        }
    }
}

pub struct CastlingSquares<'a> {
    pub empty_indices: &'a [usize],
    pub check_indices: &'a [usize],
    pub king_start_index: usize,
    pub rook_start_index: usize,
    pub rook_target_index: usize,
}

pub fn get_squares(mov: &Move) -> Result<CastlingSquares, board::Error> {
    if mov.indices() == (60, 62) {
        Ok(CastlingSquares {
            empty_indices: &[61, 62],
            check_indices: &[60, 61, 62],
            king_start_index: 60,
            rook_start_index: 63,
            rook_target_index: 61,
        })
    } else if mov.indices() == (4, 6) {
        Ok(CastlingSquares {
            empty_indices: &[5, 6],
            check_indices: &[4, 5, 6],
            king_start_index: 4,
            rook_start_index: 7,
            rook_target_index: 5,
        })
    } else if mov.indices() == (60, 58) {
        Ok(CastlingSquares {
            empty_indices: &[57, 58, 59],
            check_indices: &[58, 59, 60],
            king_start_index: 60,
            rook_start_index: 56,
            rook_target_index: 59,
        })
    } else if mov.indices() == (4, 2) {
        Ok(CastlingSquares {
            empty_indices: &[1, 2, 3],
            check_indices: &[2, 3, 4],
            king_start_index: 4,
            rook_start_index: 0,
            rook_target_index: 3,
        })
    } else {
        Err(board::Error::InvalidCastlingMove(mov.target_index))
    }
}
