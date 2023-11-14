use crate::square::*;
use crate::mov::*;
use crate::color::*;
use crate::castling::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum GameState {
    Playing,
    Draw,
    Victory(Color),
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BoardState {
    pub turn: Color,
    pub castling_state: CastlingState,
    pub ep_index: Option<usize>,
    pub halfmove_clock: u32,
    pub fullmove_count: u32,
    pub game_state: GameState,
    pub last_captured_square: Option<Square>,
    pub last_move: Option<Move>,
    pub last_ep_taken_index: Option<usize>,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Algorithm {
    Clone,
    Unmove,
}

pub trait PlayableBoard {
    fn new() -> Self;

    fn from(chars: &[char; 64]) -> Result<Self, String>
    where Self: Sized;

    fn from_fen(fen: &str) -> Result<Self, String>
    where Self: Sized;

    fn squares(&self) -> [Square; 64];
    fn state(&self) -> BoardState;

    fn to_fen(&self) -> &str;
    fn display(&self);
    fn dbg_moves(
        &self,
        moves: &Vec<Move>,
        shown_pieces: Vec<PieceKind>,
        show_castling: bool,
    ) -> Result<(), String> ;
    fn change_turn(&mut self);
    fn gen_legal_moves(&mut self) -> Result<Vec<Move>, String>;
    fn make_move(&mut self, m: &Move) -> Result<(), String>;
    fn unmake_move(&mut self) -> Result<(), String>;
    fn king_index(&self, color: Color) -> Result<usize, String>;

    fn dbg_gen_moves(&mut self) -> Result<(), String>;
    fn dbg_play_move(&mut self, m: &[usize; 2]) -> Result<(), String>;
    fn dbg_no_castle(&mut self);
    fn dbg_move_count(&mut self) -> Result<u32, String>;
    fn dbg_depth_num_positions(&mut self, depth: i32) -> Result<u32, String>;
    fn perft(&mut self, depth: i32) -> Result<u32, String>;
    fn dbg_history(&self);
    fn play_random_game(
        &mut self,
        move_limit: u32,
    ) -> Result<GameState, String>;
    fn dbg_set_algorithm(&mut self, algorithm: Algorithm);
}

use crate::implementations::*;

pub fn from_fen(fen: &str, implementation: Implementation) -> Result<impl PlayableBoard, String> {
    match implementation {
        Implementation::Original => Ok(original::Board::from_fen(fen)?),
    }
}

pub fn new(implementation: Implementation) -> impl PlayableBoard {
    match implementation {
        Implementation::Original => original::Board::new(),
    }
}
