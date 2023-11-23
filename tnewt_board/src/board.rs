// use rustc_hash::FxHashSet;
use crate::piece::Piece;
use colored::Colorize;

use std::fmt::Debug;
use std::num::ParseIntError;

use crate::castling;
use crate::color::Color;
use crate::mov::{BasicMove, CastlingMove, Move, PromotionMove};
use crate::piece::{PieceKind, Square};

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

pub trait Playable: Clone + Debug {
    #[rustfmt::skip]
    fn new() -> Self
    where Self: Sized {
        Self::from_chars(&[
            'r','n','b','q','k','b','n','r',
            'p','p','p','p','p','p','p','p',
            ' ',' ',' ',' ',' ',' ',' ',' ',
            ' ',' ',' ',' ',' ',' ',' ',' ',
            ' ',' ',' ',' ',' ',' ',' ',' ',
            ' ',' ',' ',' ',' ',' ',' ',' ',
            'P','P','P','P','P','P','P','P',
            'R','N','B','Q','K','B','N','R',
        ]).unwrap()
    }

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
    where
        Self: Sized;

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
    where
        Self: Sized;

    /// Returns the current position of the board.
    fn squares(&self) -> [Square; 64];

    /// Returns the current state of the board.
    fn state(&self) -> State;

    /// Returns the algorithm used by the board.
    fn algorithm(&self) -> Algorithm;

    /// Returns the current position and state in FEN notation.
    /// See: https://en.wikipedia.org/wiki/Forsyth-Edwards_Notation
    fn to_fen(&self) -> &str {
        let mut squares = [' '; 64];
        for (i, &square) in self.squares().iter().enumerate() {
            squares[i] = Piece::square_to_char(square);
        }
        for char in squares.iter() {
            let _ = char;
            todo!()
        }
        todo!()
    }

    /// Generates a Vec of all legal moves in the current position.
    ///
    /// # Errors
    ///
    /// This function will return an error if move generation fails
    /// (should never happen for bug-free code, eventually will be
    /// able to be unwrapped directly)
    fn gen_legal_moves(&mut self) -> Result<Vec<Move>, Error>;

    /// Prints the current position in a human-readable format.
    fn display(&self) {
        for (i, &square) in self.squares().iter().enumerate() {
            print!("{} ", Piece::display_square(square));
            if i % 8 == 7 {
                for j in i - 7..=i {
                    print!(" {}", format!("{j: >2}").black());
                }
                println!();
            }
        }
    }

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
    fn display_moves(
        &self,
        moves: &[Move],
        shown_pieces: Vec<PieceKind>,
        show_castling: bool,
    ) -> Result<(), Error> {
        BasicMove::dbg_moves(
            &moves
                .iter()
                .filter_map(|m| match m {
                    Move::BasicMove(bm) => Some(*bm),
                    _ => None,
                })
                .map(|m| {
                    if shown_pieces == &[] {
                        return Ok(Some(m));
                    }
                    let piece_kind = match self.squares()[m.start_index] {
                        Some(piece) => piece.kind,
                        None => return Err(Error::MoveEmptySquare),
                    };
                    if shown_pieces.contains(&piece_kind) {
                        return Ok(Some(m));
                    }
                    Ok(None)
                })
                .collect::<Result<Vec<Option<_>>, Error>>()?
                .into_iter()
                .filter_map(|m| m)
                .collect::<Vec<_>>(),
            &self.squares(),
        );
        if !show_castling {
            return Ok(());
        }
        CastlingMove::dbg_moves(
            &moves
                .iter()
                .filter_map(|m| match m {
                    Move::CastlingMove(cm) => Some(*cm),
                    _ => None,
                })
                .collect::<Vec<_>>(),
        );
        PromotionMove::dbg_moves(
            &moves
                .iter()
                .filter_map(|m| match m {
                    Move::PromotionMove(pm) => Some(*pm),
                    _ => None,
                })
                .collect::<Vec<_>>(),
            &self.squares(),
        );
        Ok(())
    }

    /// Change the board's current turn.
    fn change_turn(&mut self);

    /// Play `mov` on the board, and update the state and history accordingly.
    /// `mov` must be legal.
    ///
    /// # Errors
    ///
    /// This function will return an error if `move` attempts to move from an empty square
    /// or is illegal.
    fn make_move(&mut self, mov: &Move) -> Result<(), Error>;

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
    fn king_index(&self, color: Color) -> Result<usize, Error> {
        match self.squares().iter().position(|&square| {
            square
                == Some(Piece {
                    kind: PieceKind::King,
                    color,
                })
        }) {
            Some(index) => Ok(index),
            None => Err(Error::NoKing),
        }
    }

    /// Generates the board's current legal moves and displays them in a
    /// human-readable format.
    ///
    /// Only requires a mutable reference when the `unmove` algorithm is being used.
    ///
    /// # Errors
    ///
    /// This function will return an error if move generation fails.
    fn gen_and_display_moves(&mut self) -> Result<(), Error> {
        let moves = self.gen_legal_moves()?;
        self.display_moves(&moves, vec![], true)?;
        Ok(())
    }


    /// Move piece from `start_index` to `target_index`.
    ///
    /// # Errors
    ///
    /// This function will return an error if `start_index` or `target_index` are not within
    /// 0..64, if `start_index` is an empty square, or if the move is not legal.
    fn dbg_play_move(&mut self, start_index: usize, target_index: usize) -> Result<(), Error> {
        self.make_move(&BasicMove::from(&[start_index, target_index]).into())?;
        Ok(())
    }

    /// Revoke all castling rights.
    fn set_castling_state(&mut self, state: &str);

    fn set_state(&mut self, state: State);

    /// Get the current number of legal moves.
    ///
    /// Only requires a mutable reference when the `unmove` algorithm is being used.
    ///
    /// # Errors
    ///
    /// This function will return an error if move generation fails.
    fn num_legal_moves(&mut self) -> Result<usize, Error> {
        Ok(self.gen_legal_moves()?.len())
    }

    /// Get the number of possible positions after a certain `depth`.
    /// A `depth` of 0 gives 1, and a depth of 1 gives the current number of legal moves.
    ///
    /// Only requires a mutable reference when the `unmove` algorithm is being used.
    ///
    /// # Errors
    ///
    /// This function will return an error if move generation fails.
    fn depth_num_positions(&mut self, depth: i32) -> Result<u32, Error>;

    // fn depth_num_positions(&mut self, depth: i32) -> Result<u32, Error> {
    //     if depth <= 0 { return Ok(1) }
    //     let moves = self.gen_legal_moves()?;
    //     let mut num_positions: u32 = 0;
    //
    //     match self.algorithm() {
    //         Algorithm::Clone => {
    //             for mov in &moves {
    //                 let mut board = self.clone();
    //                 board.make_move(mov)?;
    //                 num_positions += board.depth_num_positions(depth - 1)?;
    //             }
    //         },
    //         Algorithm::Unmove => {
    //             for mov in &moves {
    //                 self.make_move(mov)?;
    //                 num_positions += self.depth_num_positions(depth - 1)?;
    //                 self.unmake_move()?;
    //             }
    //         },
    //     }
    //
    //     Ok(num_positions)
    // }

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
    fn perft(&mut self, depth: i32) -> Result<u32, Error> {
        if depth <= 0 {
            return Ok(0);
        }
        let moves = self.gen_legal_moves()?;

        let mut total_positions: u32 = 0;

        for mov in &moves {
            let num_moves = match self.algorithm() {
                Algorithm::Clone => {
                    let mut board = self.clone();
                    board.make_move(mov)?;
                    board.depth_num_positions(depth - 1)?

                },
                Algorithm::Unmove => {
                    self.make_move(mov)?;
                    let n = self.depth_num_positions(depth - 1)?;
                    self.unmake_move()?;
                    n
                },
            };
            total_positions += num_moves;
            println!("{mov}: {num_moves}");
        }
        println!("Total Positions: {total_positions}");
        Ok(total_positions)
    }

    /// Returns the history of the board.
    fn _history(&self) -> Vec<[Option<Piece>; 64]>;

    /// Display the position history of the board in a human-readable format.
    /// History is only stored if the `DEBUG_HISTORY` global is set to true.
    fn display_history(&self) {
        for board in &self._history() {
            for (i, &square) in board.iter().enumerate() {
                print!("{} ", Piece::display_square(square));
                if i % 8 == 7 {
                    for j in i - 7..=i {
                        print!(" {}", format!("{j: >2}").black());
                    }
                    println!();
                }
            }
            println!("{:?}", self.state().turn);
        }
    }

    /// If player has a legal move, i.e. there is some `mov`, play it, otherwise
    /// the game is over so state is updated accordingly.
    ///
    /// # Arguments
    /// * `mov` - The move to play, if one exists. Must be legal.
    ///
    /// # Errors
    ///
    /// This function will return an error if move generation fails,
    /// if `mov` is illegal, or if the player has no king.
    fn play_legal_move(&mut self, mov: Option<&Move> ) -> Result<(), Error>;

    /// Play a random game up to `move_limit` moves.
    /// Will leave the board in the last position of the game.
    ///
    /// # Errors
    ///
    /// This function will return an error if move generation fails
    // fn play_random_game(&mut self, move_limit: u32) -> Result<GameState, Error>;
    fn play_random_game(
        &mut self,
        move_limit: u32,
    ) -> Result<GameState, Error> {
        use rand::{seq::SliceRandom, thread_rng};
        for _ in 0..move_limit {
            let moves = self.gen_legal_moves()?;

            let mut rng = thread_rng();
            let mov = moves.choose(&mut rng);
            self.play_legal_move(mov)?;
            if self.state().game_state != GameState::Playing { break }
        }
        Ok(self.state().game_state)
    }

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
    fn set_algorithm(&mut self, algorithm: Algorithm);
}

#[must_use]
pub fn from_fen<B: Playable>(fen: &str) -> Result<B, Error> {
    B::from_fen(fen)
}

#[must_use]
pub fn new<B: Playable>() -> B {
    B::new()
}

#[must_use]
pub fn from_chars<B: Playable>(chars: &[char; 64]) -> Result<impl Playable, Error> {
    B::from_chars(chars)
}
