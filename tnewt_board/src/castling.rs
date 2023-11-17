use crate::mov::{CastlingMove, Move};
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
            Rights::Neither   => vec![],
            Rights::Kingside  => vec![kingside],
            Rights::Queenside => vec![queenside],
            Rights::Both      => vec![kingside, queenside],
        }
    }
    pub fn revoke(&mut self, right: Rights) {
        match right {
            Rights::Neither => return,
            Rights::Both => { *self = Rights::Neither; return },
            _ => (),
        }
        match self {
            Rights::Neither => (),
            Rights::Kingside => match right {
                Rights::Kingside =>  *self = Rights::Neither ,
                _ => (),
            },
            Rights::Queenside => match right {
                Rights::Queenside => *self = Rights::Neither,
                _ => (),
            },
            Rights::Both => match right {
                Rights::Kingside => *self = Rights::Queenside,
                Rights::Queenside => *self = Rights::Kingside,
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
