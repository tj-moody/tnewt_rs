// use rustc_hash::FxHashSet;

use std::num::ParseIntError;

use crate::piece::{PieceKind, Square};
use crate::mov::Move;
use crate::color::Color;
use crate::castling;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum GameState {
    Playing,
    Draw,
    Victory(Color),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidCoordinate(String),
    InvalidFenBoardLength(String),
    InvalidHalfmoveStr(ParseIntError),
    InvalidFullmoveStr(ParseIntError),
    PieceFromEmptySquare,
    MoveEmptySquare,
    InvalidPieceChar(char),
    NoKing,
    UndoFromFirstMove,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct State {
    pub turn: Color,
    pub castling_state: castling::State,
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

// TODO: Only expose externally necessary functions
// TODO: Unwrap functions that (if move logic is correct) should never error,
//       i.e. move generation functions
//
//      "This function will return an error if move generation fails"

pub trait Playable {
    fn new() -> Self;

    /// Returns a playable board at the first move, except with position specified by `chars`.
    ///
    /// Each element in `chars` may be one of ('k', 'q', 'r', 'b', 'n', 'p', ' '),
    /// with uppercase denoting white, lowercase denoting black, and ' ' denoting empty.
    ///
    /// # Errors
    ///
    /// This function will return an error if `chars` contains an invalid character.
    ///
    /// # Examples
    /// ```
    /// let mut board = Playable::from_chars(&[
    ///     'r','n','b','q','k','b','n','r',
    ///     'p','p','p','p','p','p','p','p',
    ///     ' ',' ',' ',' ',' ',' ',' ',' ',
    ///     ' ',' ',' ',' ',' ',' ',' ',' ',
    ///     ' ',' ',' ',' ',' ',' ',' ',' ',
    ///     ' ',' ',' ',' ',' ',' ',' ',' ',
    ///     'P','P','P','P','P','P','P','P',
    ///     'R','N','B','Q','K','B','N','R',
    /// ])?;
    /// ```
    fn from_chars(chars: &[char; 64]) -> Result<Self, Error>
    where Self: Sized;

    /// Returns a playable board.
    ///
    /// The state of the board is determined in full by `fen`. Although the board
    /// may initialized beyond the first move, it will have no history.
    ///
    /// # Arguments
    /// * `fen` - A string slice representing the board in FEN notation.
    ///
    /// # Errors
    ///
    /// This function will return an error if `fen` is not a valid FEN string.
    ///
    /// See: https://en.wikipedia.org/wiki/Forsyth-Edwards_Notation
    fn from_fen(fen: &str) -> Result<Self, Error>
    where Self: Sized;

    /// Returns the current position of the board.
    fn squares(&self) -> [Square; 64];

    /// Returns the current state of the board.
    fn state(&self) -> State;

    /// Returns the current position and state in FEN notation.
    /// See: https://en.wikipedia.org/wiki/Forsyth-Edwards_Notation
    fn to_fen(&self) -> &str;

    /// Prints the current position in a human-readable format.
    fn display(&self);

    /// Displays a list of moves on the current board in a human-readable format.
    ///
    /// # Arguments
    /// * `moves` - A slice of the moves to be displayed.
    /// * `shown_pieces` - A Vec of which kinds pieces to show moves for,
    /// or all pieces if empty.
    /// * `show_castling` - Whether or not to show castling moves.
    ///
    /// # Errors
    ///
    /// This function will return an error if a move in `moves` attempts to move
    /// from an empty square.
    fn dbg_moves(
        &self,
        moves: &[Move],
        shown_pieces: Vec<PieceKind>,
        show_castling: bool,
    ) -> Result<(), Error> ;

    /// Change the board's current turn.
    fn change_turn(&mut self);

    /// Play `move` on the board, and update the state and history accordingly.
    ///
    /// # Errors
    ///
    /// This function will return an error if `move` attempts to move from an empty square.
    fn make_move(&mut self, m: &Move) -> Result<(), Error>;

    /// Undo the board's most recent move.
    ///
    /// # Errors
    ///
    /// This function will return an error if the board is on its first move.
    fn unmake_move(&mut self) -> Result<(), Error>;

    /// Returns the index of the `color`'s king.
    ///
    /// # Errors
    ///
    /// This function will return an error if player `color` does not have a king.
    fn king_index(&self, color: Color) -> Result<usize, Error>;

    /// Generates the board's current legal moves and displays them in a
    /// human-readable format.
    ///
    /// Only requires a mutable reference when the `unmove` algorithm is being used.
    ///
    /// # Errors
    ///
    /// This function will return an error if move generation fails.
    fn dbg_gen_moves(&mut self) -> Result<(), Error>;

    /// Move piece from `start_index` to `target_index`.
    ///
    /// # Errors
    ///
    /// This function will return an error if `start_index` or `target_index` are not within
    /// 0..64, if `start_index` is an empty square, or if the move is not legal.
    fn dbg_play_move(&mut self, start_index: usize, target_index: usize) -> Result<(), Error>;

    /// Revoke all castling rights.
    fn dbg_no_castle(&mut self);

    /// Get the current number of legal moves.
    ///
    /// Only requires a mutable reference when the `unmove` algorithm is being used.
    ///
    /// # Errors
    ///
    /// This function will return an error if move generation fails.
    fn dbg_move_count(&mut self) -> Result<usize, Error>;

    /// Get the number of possible positions after a certain `depth`.
    /// A `depth` of 0 gives 1, and a depth of 1 gives the current number of legal moves.
    ///
    /// Only requires a mutable reference when the `unmove` algorithm is being used.
    ///
    /// # Errors
    ///
    /// This function will return an error if move generation fails.
    fn dbg_depth_num_positions(&mut self, depth: i32) -> Result<u32, Error>;

    /// A debugging tool that displays the number of possible positions after `depth`
    /// for each legal move in the current position.
    ///
    /// Only requires a mutable reference when the `unmove` algorithm is being used.
    ///
    /// # Errors
    ///
    /// This function will return an error if move generation fails.
    ///
    /// See: https://www.chessprogramming.org/Perft
    fn perft(&mut self, depth: i32) -> Result<u32, Error>;

    /// Display the position history of the board in a human-readable format.
    /// History is only stored if the `DEBUG_HISTORY` global is set to true.
    fn dbg_history(&self);

    /// Play a random game up to `move_limit` moves.
    /// Will leave the board in the last position of the game.
    ///
    /// # Errors
    ///
    /// This function will return an error if move generation fails
    fn play_random_game(
        &mut self,
        move_limit: u32,
    ) -> Result<GameState, Error>;

    /// Set the algorith of the board to `Unmove` or `Clone`.
    /// * `Clone` will clone the current board before making a move to test if that
    /// move leaves the player in check.
    /// * `Unmove` makes a move on the current board to test if that move leaves the
    /// player in check, before undoing that move.
    ///
    /// Clone requires cloning the entire board, but does not require a mutable reference,
    /// and therefore can be parallelized, and does not require storing state history.
    /// Unmove never clones the board, so can be faster on a single-thread, but requires
    /// passing a mutable reference everywhere, and requires storing a full state history
    /// for the board.
    fn dbg_set_algorithm(&mut self, algorithm: Algorithm);
}


pub fn from_fen<B: Playable>(fen: &str) -> Result<B, Error> {
    B::from_fen(fen)
}

#[must_use]
pub fn new<B: Playable>() -> B {
    B::new()
}

pub fn from_chars<B: Playable>(chars: &[char; 64]) -> Result<impl Playable, Error> {
    B::from_chars(chars)
}
