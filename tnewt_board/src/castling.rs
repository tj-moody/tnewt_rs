use crate::mov::*;
use crate::color::*;

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
    pub fn gen_moves(&self, color: &Color) -> Result<Vec<Move>, String> {
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
            CastlingRights::Neither   => Ok(vec![]),
            CastlingRights::Kingside  => Ok(vec![kingside]),
            CastlingRights::Queenside => Ok(vec![queenside]),
            CastlingRights::Both      => Ok(vec![kingside, queenside]),
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
pub struct CastlingState {
    pub white: CastlingRights,
    pub black: CastlingRights,
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
    pub fn get_moves(&self, color: &Color) -> Result<Vec<Move>, String> {
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
