// use rustc_hash::FxHashSet;

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

    fn from_chars(chars: &[char; 64]) -> Result<Self, String>
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

// TODO: Convert all of these into a macro

use crate::implementations::*;
pub fn from_fen(fen: &str) -> Result<impl PlayableBoard, String> {
    // original::Board::from_fen(fen)
    mut_pass::Board::from_fen(fen)
    // hash_set::Board::from_fen(fen)
}

pub fn new() -> impl PlayableBoard {
    // original::Board::new()
    mut_pass::Board::new()
    // hash_set::Board::new()
}

pub fn from_chars(chars: &[char; 64]) -> Result<impl PlayableBoard, String> {
    // original::Board::from_chars(chars)
    mut_pass::Board::from_chars(chars)
    // hash_set::Board::from_chars(chars)
}
