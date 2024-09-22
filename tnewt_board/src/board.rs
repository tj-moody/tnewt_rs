use color_eyre::eyre::Result;
use colored::Colorize;

use crate::{castling, color, coordinate, mov, piece};

use color::Color;
use coordinate::Coordinate;
use mov::Move;
use piece::{Kind, Piece, Square};

use crate::magic_numbers::{
    get_pawn_threat_offsets, get_threat_pieces, DIRECTION_OFFSETS, KNIGHT_THREAT_INDICES,
    SQUARES_TO_EDGE,
};

use std::fmt::Debug;
use std::num::ParseIntError;

pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const STARTING_POSITION: &[char; 64] = &[
    'r', 'n', 'b', 'q', 'k', 'b', 'n', 'r', //  0  1  2  3  4  5  6  7
    'p', 'p', 'p', 'p', 'p', 'p', 'p', 'p', //  8  9 10 11 12 13 14 15
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 16 17 18 19 20 21 22 23
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 24 25 26 27 28 29 30 31
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 32 33 34 35 36 37 38 39
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 40 41 42 43 44 45 46 47
    'P', 'P', 'P', 'P', 'P', 'P', 'P', 'P', // 48 49 50 51 52 53 54 55
    'R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R', // 56 57 58 59 60 61 62 63
];
pub const EMPTY_POSITION: &[char; 64] = &[
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', //  0  1  2  3  4  5  6  7
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', //  8  9 10 11 12 13 14 15
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 16 17 18 19 20 21 22 23
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 24 25 26 27 28 29 30 31
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 32 33 34 35 36 37 38 39
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 40 41 42 43 44 45 46 47
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 48 49 50 51 52 53 54 55
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 56 57 58 59 60 61 62 63
];

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum GameState {
    Playing,
    Draw,
    Victory(Color),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct KingIndices {
    pub white: usize,
    pub black: usize,
}

impl KingIndices {
    #[must_use]
    pub fn get(&self, color: Color) -> usize {
        match color {
            Color::White => self.white,
            Color::Black => self.black,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidCoordinate(String),
    InvalidFenBoardLength(String),
    InvalidHalfmoveStr(ParseIntError),
    InvalidFullmoveStr(ParseIntError),
    InvalidColorStr(String),
    InvalidPieceChar(char),
    PieceFromEmptySquare,
    MoveEmptySquare,
    MoveOppositeColor,
    NoKing,
    UndoFromFirstMove,
    InvalidCastlingMove(usize),
    InvalidDirectionIndex(usize),
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct State {
    turn: Color,
    castling_state: castling::State,
    ep_index: Option<usize>,
    halfmove_clock: u32,
    fullmove_count: u32,
    game_state: GameState,
    last_captured_square: Option<Square>,
    last_move: Option<Move>,
    last_ep_taken_index: Option<usize>,
    king_indices: KingIndices,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Algorithm {
    Clone,
    Unmove,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Board {
    pub squares: [Option<Piece>; 64],
    pub state: State,
    pub state_history: Vec<State>,
    pub history: Vec<[Option<Piece>; 64]>,
    pub algorithm: Algorithm,
    pub store_history: bool,
}

impl Board {
    fn slider_gen_moves(&self, moves: &mut Vec<Move>, start_index: usize, piece: Piece) {
        let start_square = self.squares[start_index];

        let mut start_dir_index = 0;
        let mut end_dir_index = 8;
        let mut branch_length = 8;

        match piece.kind {
            Kind::Bishop => start_dir_index = 4,
            Kind::Rook => end_dir_index = 4,
            Kind::King => branch_length = 1,
            _ => (),
        }

        // for direction_index in start_dir_index..end_dir_index {
        for (direction_index, offset) in DIRECTION_OFFSETS
            .iter()
            .enumerate()
            .take(end_dir_index)
            .skip(start_dir_index)
        {
            for n in 0..branch_length.min(SQUARES_TO_EDGE[start_index][direction_index]) {
                let target_index: usize = (start_index as i32 + (offset * (n + 1))) as usize;

                let target_square = self.squares[target_index];

                if Piece::is_same_color(start_square, target_square) {
                    break;
                }

                moves.push(Move::new(start_index, target_index));

                if target_square.is_some() {
                    break;
                }
            }
        }
    }

    fn knight_gen_moves(&self, moves: &mut Vec<Move>, start_index: usize) {
        let start_square = self.squares[start_index];
        for &target_index in KNIGHT_THREAT_INDICES[start_index] {
            let target_square = self.squares[target_index];
            if !Piece::is_same_color(start_square, target_square) {
                moves.push(Move::new(start_index, target_index));
            }
        }
    }

    fn pawn_gen_moves(&self, moves: &mut Vec<Move>, start_index: usize) {
        let start_square = self.squares[start_index];

        let rank = start_index / 8 % 8;
        let file = start_index % 8;

        let color = start_square.expect("Moved from empty square").color;

        let offsets = match color {
            Color::White => [-8, -16, -9, -7],
            Color::Black => [8, 16, 7, 9],
        };
        let invalid_ranks = match color {
            Color::White => [0, 1],
            Color::Black => [7, 6],
        };
        let starting_rank = match color {
            Color::White => 6,
            Color::Black => 1,
        };
        if rank == invalid_ranks[0] {
            return;
        }

        if rank != invalid_ranks[0] {
            let target_index: usize = (start_index as i32 + offsets[0]) as usize;
            let target_square = self.squares[target_index];
            if target_square.is_none() {
                let mov = Move::new(start_index, target_index);
                if rank == invalid_ranks[1] {
                    for promotion_move in mov.promotion_moves() {
                        moves.push(promotion_move);
                    }
                } else {
                    moves.push(mov);
                }
                if rank != invalid_ranks[1] && rank == starting_rank {
                    let target_index: usize = (start_index as i32 + offsets[1]) as usize;
                    let target_square = self.squares[target_index];
                    if target_square.is_none() {
                        moves.push(Move::new(start_index, target_index));
                    }
                }
            }
        }
        if file != 0 {
            let target_index: usize = (start_index as i32 + offsets[2]) as usize;
            let target_square = self.squares[target_index];
            if (!Piece::is_same_color(start_square, target_square) && target_square.is_some())
                || Some(target_index) == self.state.ep_index
            {
                let mov = Move::new(start_index, target_index);
                if rank == invalid_ranks[1] {
                    for promotion_move in mov.promotion_moves() {
                        moves.push(promotion_move);
                    }
                } else {
                    moves.push(mov);
                }
            }
        }
        if file != 7 {
            let target_index: usize = (start_index as i32 + offsets[3]) as usize;
            let target_square = self.squares[target_index];
            if (!Piece::is_same_color(start_square, target_square) && target_square.is_some())
                || Some(target_index) == self.state.ep_index
            {
                let mov = Move::new(start_index, target_index);
                if rank == invalid_ranks[1] {
                    for promotion_move in mov.promotion_moves() {
                        moves.push(promotion_move);
                    }
                } else {
                    moves.push(mov);
                }
            }
        }
    }

    fn gen_castling_moves(&self, moves: &mut Vec<Move>) {
        self.state
            .castling_state
            .get_moves(&self.state.turn)
            .into_iter()
            .for_each(|mov| {
                let castling_squares = castling::get_squares(&mov)
                    .expect("Generate invalid castling move from `castling_state.gen_moves()");
                for &empty_index in castling_squares.empty_indices {
                    if self.squares[empty_index].is_some() {
                        return;
                    };
                }
                match self.squares[castling_squares.king_start_index] {
                    Some(piece) => match piece.kind {
                        Kind::King => {
                            if piece.color != self.state.turn {
                                return;
                            }
                        }
                        _ => return,
                    },
                    None => return,
                };
                match self.squares[castling_squares.rook_start_index] {
                    Some(piece) => match piece.kind {
                        Kind::Rook => {
                            if piece.color != self.state.turn {
                                return;
                            }
                        }
                        _ => return,
                    },
                    None => return,
                };
                moves.push(mov);
            });
    }

    fn gen_pseudo_legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(50);
        for index in 0..64 {
            let square = self.squares[index];
            if let Some(piece) = square {
                if piece.color != self.state.turn {
                    continue;
                }
                match piece.kind {
                    Kind::Bishop | Kind::Rook | Kind::Queen | Kind::King => {
                        self.slider_gen_moves(&mut moves, index, piece)
                    }
                    Kind::Knight => self.knight_gen_moves(&mut moves, index),
                    Kind::Pawn => self.pawn_gen_moves(&mut moves, index),
                };
            }
        }
        self.gen_castling_moves(&mut moves);

        moves
    }

    /// Only checks if the color whose turn it is has their piece at `index` being attacked
    fn is_attacked(&mut self, indices: &[usize], king_color: Color) -> bool {
        for &start_index in indices {
            for &knight_index in KNIGHT_THREAT_INDICES[start_index] {
                if let Some(piece) = self.squares[knight_index] {
                    if piece.kind == Kind::Knight && piece.color != king_color {
                        return true;
                    }
                }
            }

            let pawn_threat_offsets = get_pawn_threat_offsets(king_color);
            for (direction_index, offset) in DIRECTION_OFFSETS.iter().enumerate() {
                let threat_pieces = get_threat_pieces(direction_index)
                    .expect("DIRECTION_OFFSETS yields valid direction offsets");
                for n in 0..SQUARES_TO_EDGE[start_index][direction_index] {
                    let target_index: usize = (start_index as i32 + (offset * (n + 1))) as usize;

                    let target_square = self.squares[target_index];

                    match target_square {
                        Some(piece) => {
                            if piece.color == king_color {
                                break;
                            }
                            if threat_pieces.contains(&piece.kind) {
                                return true;
                            }
                            if n == 0 {
                                if piece.kind == Kind::Pawn && pawn_threat_offsets.contains(offset)
                                {
                                    return true;
                                }
                                if piece.kind == Kind::King {
                                    return true;
                                }
                            }
                            break;
                        }
                        None => continue,
                    }
                }
            }
        }
        false
    }

    fn gen_king_indices(squares: &[Option<Piece>; 64]) -> Result<KingIndices, Error> {
        let mut white: usize = 64;
        let mut black: usize = 64;
        for (i, square) in squares.iter().enumerate() {
            if let Some(piece) = square {
                if piece.kind == Kind::King {
                    match piece.color {
                        Color::White => white = i,
                        Color::Black => black = i,
                    }
                }
            }
        }
        if white == 64 || black == 64 {
            return Err(Error::NoKing);
        }
        Ok(KingIndices { white, black })
    }

    /// Generates a Vec of all legal moves in the current position.
    pub fn gen_legal_moves(&mut self) -> Vec<Move> {
        let mut moves = self.gen_pseudo_legal_moves();
        moves.retain(|mov| {
            let is_castling = mov.is_castling(&self.squares).unwrap();

            match self.algorithm {
                Algorithm::Clone => {
                    let mut board = self.clone();
                    board.make_move(mov);

                    let king_index = board.state.king_indices.get(board.state.turn.opposite());
                    let check_indices = if is_castling {
                        castling::get_squares(mov).unwrap().check_indices.to_vec()
                    } else {
                        vec![king_index]
                    };

                    !board.is_attacked(&check_indices, board.state.turn.opposite())
                }
                Algorithm::Unmove => {
                    self.make_move(mov);

                    let king_index = self.state.king_indices.get(self.state.turn.opposite());
                    let check_indices = if is_castling {
                        castling::get_squares(mov).unwrap().check_indices.to_vec()
                    } else {
                        vec![king_index]
                    };

                    let is_attacked = self.is_attacked(&check_indices, self.state.turn.opposite());
                    self.unmake_move();
                    !is_attacked
                }
            }
        });
        moves
    }

    /// Returns a playable board at the first move, except with position specified by [`chars`.]
    ///
    /// Each element in [`chars`] may be one of ('k', 'q', 'r', 'b', 'n', 'p', ' '),
    /// with uppercase denoting white, lowercase denoting black, and ' ' denoting empty.
    ///
    /// # Errors
    ///
    /// This function will return an error if [`chars`] contains an invalid character.
    ///
    /// # Examples
    /// ```
    /// use tnewt_board::board::Board;
    /// let mut board = Board::from_chars(&[
    ///     'r','n','b','q','k','b','n','r',
    ///     'p','p','p','p','p','p','p','p',
    ///     ' ',' ',' ',' ',' ',' ',' ',' ',
    ///     ' ',' ',' ',' ',' ',' ',' ',' ',
    ///     ' ',' ',' ',' ',' ',' ',' ',' ',
    ///     ' ',' ',' ',' ',' ',' ',' ',' ',
    ///     'P','P','P','P','P','P','P','P',
    ///     'R','N','B','Q','K','B','N','R',
    /// ]);
    /// ```
    pub fn from_chars(chars: &[char; 64]) -> Result<Self, Error> {
        let mut squares: [Option<Piece>; 64] = [None; 64];
        for (i, piece_char) in chars.iter().enumerate() {
            let piece = match piece_char.to_ascii_lowercase() {
                'k' => Some(Kind::King),
                'q' => Some(Kind::Queen),
                'r' => Some(Kind::Rook),
                'b' => Some(Kind::Bishop),
                'n' => Some(Kind::Knight),
                'p' => Some(Kind::Pawn),
                ' ' => None,
                _ => return Err(Error::InvalidPieceChar(*piece_char)),
            };
            squares[i] = match piece_char {
                'k' | 'q' | 'r' | 'b' | 'n' | 'p' => Piece {
                    kind: piece.unwrap(),
                    color: Color::Black,
                }
                .into(),
                'K' | 'Q' | 'R' | 'B' | 'N' | 'P' => Piece {
                    kind: piece.unwrap(),
                    color: Color::White,
                }
                .into(),
                ' ' => None,
                _ => return Err(Error::InvalidPieceChar(*piece_char)),
            };
        }
        Ok(Board {
            squares,
            state: State {
                turn: Color::White,
                castling_state: castling::State {
                    white: castling::Rights::Both,
                    black: castling::Rights::Both,
                },
                ep_index: None,
                halfmove_clock: 0,
                fullmove_count: 1,
                game_state: GameState::Playing,
                last_captured_square: None,
                last_move: None,
                last_ep_taken_index: None,
                king_indices: Board::gen_king_indices(&squares)?,
            },
            state_history: vec![],
            history: vec![],
            algorithm: Algorithm::Clone,
            store_history: false,
        })
    }

    /// Returns a playable board.
    ///
    /// The state of the board is determined in full by [`fen`]. Although the board
    /// may initialized beyond the first move, it will have no history.
    ///
    /// # Arguments
    /// * [`fen`] - A string slice representing the board in FEN notation.
    ///
    /// # Errors
    ///
    /// This function will return an error if [`fen`] is not a valid FEN string.
    ///
    /// See: [Forsyth-Edwards_Notation](https://en.wikipedia.org/wiki/Forsyth-Edwards_Notation)
    pub fn from_fen(fen: &str) -> Result<Self, Error> {
        let mut squares: Vec<char> = vec![];
        let mut iter = fen.split(' ');
        let pieces = iter.next().expect("Invalid FEN");
        let turn = iter.next().expect("Invalid FEN");
        let castling_rights = iter.next().expect("Invalid FEN");
        let ep_square = iter.next().expect("Invalid FEN");
        let halfmove_clock = iter.next().expect("Invalid FEN");
        let fullmove_count = iter.next().expect("Invalid FEN");
        for row in pieces.split('/') {
            for c in row.chars() {
                match c {
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                        squares.append(&mut vec![' '; c.to_digit(10).unwrap() as usize]);
                    }
                    'k' | 'q' | 'r' | 'b' | 'n' | 'p' | 'K' | 'Q' | 'R' | 'B' | 'N' | 'P' => {
                        squares.append(&mut vec![c]);
                    }
                    _ => return Err(Error::InvalidPieceChar(c)),
                }
            }
        }

        let array: [char; 64] = match squares.try_into() {
            Ok(some) => some,
            Err(_) => return Err(Error::InvalidFenBoardLength(fen.to_string())),
        };

        let mut board: Board = Board::from_chars(&array)?;
        board.state.turn = Color::from(turn)?;
        board.state.castling_state = castling::State::from(castling_rights);
        board.state.ep_index = Coordinate::from(ep_square)?.map(Coordinate::into_index);
        board.state.halfmove_clock = match halfmove_clock.parse::<u32>() {
            Ok(v) => v,
            Err(e) => return Err(Error::InvalidHalfmoveStr(e)),
        };
        board.state.fullmove_count = match fullmove_count.parse::<u32>() {
            Ok(v) => v,
            Err(e) => return Err(Error::InvalidFullmoveStr(e)),
        };
        Ok(board)
    }

    /// Change the board's current turn.
    fn change_turn(&mut self) {
        self.state.turn = self.state.turn.opposite();
    }

    /// Play [`mov`] on the board, and update the state and history accordingly.
    /// [`mov`] must be legal.
    ///
    /// # Errors
    ///
    /// This function will return an error if [`mov`] attempts to move from an empty square
    /// or is illegal.
    pub fn make_move(&mut self, mov: &Move) {
        let moving_piece = self.squares[mov.start_index].expect("Move empty square");
        let captured_square = self.squares[mov.target_index];

        if self.algorithm == Algorithm::Unmove {
            self.state_history.push(self.state);
        }
        let color = self.state.turn;

        // The index of the pawn being captured via en passant, if any
        let ep_taken_index: Option<usize> = (|| {
            if let Some(index) = self.state.ep_index {
                if mov.target_index != index {
                    return None;
                }

                if moving_piece.kind != Kind::Pawn {
                    return None;
                }
                return Some(match color {
                    Color::White => mov.target_index + 8,
                    Color::Black => mov.target_index - 8,
                });
            }
            None
        })();

        if self.algorithm == Algorithm::Unmove {
            self.state.last_ep_taken_index = ep_taken_index;
            self.state.last_move = Some(*mov);
            self.state.last_captured_square = Some(self.squares[mov.target_index]);
        }

        self.state.ep_index = None;
        match captured_square {
            Some(piece) => {
                match piece.kind {
                    Kind::King => {}
                    Kind::Rook => {
                        match mov.target_index {
                            0 | 56 => self
                                .state
                                .castling_state
                                .revoke(castling::Rights::Queenside, &color.opposite()),
                            7 | 63 => self
                                .state
                                .castling_state
                                .revoke(castling::Rights::Kingside, &color.opposite()),
                            _ => (),
                        };
                    }
                    _ => (),
                };
                self.state.halfmove_clock = 0;
            }
            None => {
                if ep_taken_index.is_none() {
                    self.state.halfmove_clock += 1;
                }
            }
        };

        let mut is_castling = false;
        match moving_piece.kind {
            Kind::King => {
                self.state
                    .castling_state
                    .revoke(castling::Rights::Both, &self.state.turn);
                is_castling = mov
                    .is_castling(&self.squares)
                    .expect("Square containing king is non-empty");
                match self.state.turn {
                    Color::White => self.state.king_indices.white = mov.target_index,
                    Color::Black => self.state.king_indices.black = mov.target_index,
                }
            }
            Kind::Rook => match mov.start_index {
                0 | 56 => self
                    .state
                    .castling_state
                    .revoke(castling::Rights::Queenside, &color),
                7 | 63 => self
                    .state
                    .castling_state
                    .revoke(castling::Rights::Kingside, &color),
                _ => (),
            },
            Kind::Pawn => {
                self.state.halfmove_clock = 0;
                let offset: isize = mov.target_index as isize - mov.start_index as isize;
                self.state.ep_index = match offset {
                    -16 => Some(mov.start_index - 8),
                    16 => Some(mov.start_index + 8),
                    _ => None,
                };
            }
            _ => (),
        };
        if self.state.halfmove_clock >= 50 {
            self.state.game_state = GameState::Draw;
        }

        if self.store_history {
            self.history.push(self.squares);
        }

        if let Some(pawn_index) = ep_taken_index {
            self.squares[pawn_index] = None;
        }
        self.squares[mov.target_index] = self.squares[mov.start_index];
        self.squares[mov.start_index] = None;

        if is_castling {
            let castling_squares =
                castling::get_squares(mov).expect("`is_castling` guarantees moves is castling");
            self.squares[castling_squares.rook_target_index] = Some(Piece {
                kind: Kind::Rook,
                color,
            });
            self.squares[castling_squares.rook_start_index] = None;
        }
        if let Some(kind) = mov.promotion_kind {
            self.squares[mov.target_index] = Some(Piece { kind, color });
            self.squares[mov.start_index] = None;
        }

        if self.state.turn == Color::Black {
            self.state.fullmove_count += 1;
        }
        self.change_turn();
    }

    /// Undo the board's most recent move.
    ///
    /// # Errors
    ///
    /// This function will return an error if the board is on its first move.
    pub fn unmake_move(&mut self) {
        let last_captured_square = self
            .state
            .last_captured_square
            .expect("Undo from first move");
        let last_move = self.state.last_move.expect("Undo from first move");
        let last_ep_taken_index = self.state.last_ep_taken_index;

        self.state = self.state_history.pop().expect("Undo from first move");
        // Color of the player who made the move being undone
        let color = self.state.turn;

        if self.store_history {
            self.history.pop();
        }

        if let Some(index) = last_ep_taken_index {
            self.squares[index] = Some(Piece {
                kind: Kind::Pawn,
                color: self.state.turn.opposite(),
            });
        }

        let last_moved_square = self.squares[last_move.target_index];

        self.squares[last_move.target_index] = last_captured_square;
        if last_move.promotion_kind.is_some() {
            self.squares[last_move.start_index] = Some(Piece {
                kind: Kind::Pawn,
                color,
            });
        } else {
            self.squares[last_move.start_index] = last_moved_square;
        }

        // Can use `is_castling` method because moving piece (king) has
        // already been reset
        if last_move
            .is_castling(&self.squares)
            .expect("Last move must be a valid move")
        {
            let castling_squares =
                castling::get_squares(&last_move).expect("Last move was castling");
            self.squares[castling_squares.rook_start_index] = Some(Piece {
                kind: Kind::Rook,
                color,
            });
            self.squares[castling_squares.rook_target_index] = None;
        }
    }

    /// Set the algorith of the board to [`Unmove`] or [`Clone`.]
    /// * [`Clone`] will clone the current board before making a move to test if that
    /// move leaves the player in check.
    /// * [`Unmove`] makes a move on the current board to test if that move leaves the
    /// player in check, before undoing that move.
    ///
    /// Clone requires cloning the entire board, but does not require a mutable reference,
    /// and therefore can be parallelized, and does not require storing state history.
    /// Unmove never clones the board, so can be faster on a single-thread, but requires
    /// passing a mutable reference everywhere, and requires storing a full state history
    /// for the board.
    pub fn set_algorithm(&mut self, algorithm: Algorithm) {
        self.algorithm = algorithm;
    }

    pub fn set_castling_state(&mut self, rights: &str) {
        self.state.castling_state = castling::State::from(rights);
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    /// If player has a legal move, i.e. there is some [`mov`,] play it, otherwise
    /// the game is over so state is updated accordingly.
    ///
    /// This function does not actually mutate self, as it calls `make_move`
    /// and `unmake_move` sequentially, without mutating anywhere else.
    ///
    /// # Arguments
    /// * [`mov`] - The move to play, if one exists. Must be legal.
    ///
    /// # Errors
    ///
    /// This function will return an error if move generation fails,
    /// if [`mov`] is illegal, or if the player has no king.
    ///
    pub fn play_legal_move(&mut self, mov: Option<&Move>) {
        if let Some(mov) = mov {
            self.make_move(mov)
        } else {
            let king_index = self.state.king_indices.get(self.state.turn);
            if self.is_attacked(&[king_index], self.state.turn) {
                self.state.game_state = GameState::Victory(self.state.turn.opposite());
            } else {
                self.state.game_state = GameState::Draw;
            }
        };
    }

    pub fn squares(&self) -> [Option<Piece>; 64] {
        self.squares
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn algorithm(&self) -> Algorithm {
        self.algorithm
    }

    /// Get the number of possible positions after a certain [`depth`.]
    /// A [`depth`] of 0 gives 1, and a depth of 1 gives the current number of legal moves.
    ///
    /// Only requires a mutable reference when the [`unmove`] algorithm is being used.
    pub fn depth_num_positions(&mut self, depth: i32) -> u32 {
        if depth <= 0 {
            return 1;
        }
        let moves = self.gen_legal_moves();
        let mut num_positions: u32 = 0;

        match self.algorithm() {
            Algorithm::Clone => {
                for mov in &moves {
                    let mut board = self.clone();
                    board.make_move(mov);
                    num_positions += board.depth_num_positions(depth - 1);
                }
            }
            Algorithm::Unmove => {
                for mov in &moves {
                    self.make_move(mov);
                    num_positions += self.depth_num_positions(depth - 1);
                    self.unmake_move();
                }
            }
        }

        num_positions
    }

    // TODO: Unwrap functions that (if move logic is correct) should never error,
    //       i.e. move generation functions
    //
    //      "This function will return an error if move generation fails"

    /// Creates a new [`Board`].
    ///
    /// # Panics
    ///
    /// Panics if .
    pub fn new() -> Self {
        Self::from_chars(&[
            'r', 'n', 'b', 'q', 'k', 'b', 'n', 'r', 'p', 'p', 'p', 'p', 'p', 'p', 'p', 'p', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'P', 'P', 'P',
            'P', 'P', 'P', 'P', 'P', 'R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R',
        ])
        .unwrap()
    }

    /// Returns the current position and state in FEN notation.
    /// See: [Forsyth-Edwards_Notation](https://en.wikipedia.org/wiki/Forsyth-Edwards_Notation)
    pub fn to_fen(&self) -> &str {
        let mut squares = [' '; 64];
        for (i, square) in self.squares().iter().enumerate() {
            squares[i] = Piece::square_to_char(square);
        }
        for char in &squares {
            let _ = char;
            todo!()
        }
        todo!()
    }

    /// Prints the current position in a human-readable format.
    pub fn display(&self) {
        for (i, square) in self.squares().iter().enumerate() {
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
    /// * [`moves`] - A slice of the moves to be displayed.
    /// * [`shown_pieces`] - A Vec of which kinds pieces to show moves for,
    /// or all pieces if empty.
    /// * [`show_castling`] - Whether or not to show castling moves.
    ///
    /// # Errors
    ///
    /// This function will return an error if a move in [`moves`] attempts to move
    /// from an empty square.
    pub fn display_moves(&self, moves: &[Move], shown_pieces: Vec<Kind>) {
        Move::dbg_moves(
            moves
                .iter()
                .map(|&m| {
                    if shown_pieces.is_empty() {
                        return Some(m);
                    }
                    let piece_kind = self.squares()[m.start_index]
                        .expect("Generated moves were legal")
                        .kind;
                    if shown_pieces.contains(&piece_kind) {
                        return Some(m);
                    }
                    None
                })
                .flatten()
                .collect::<Vec<_>>()
                .as_slice(),
            &self.squares(),
        );
    }

    /// Returns the index of the [`color`'s] king.
    ///
    /// # Errors
    ///
    /// This function will return an error if player [`color`] does not have a king.
    pub fn king_index(&self, color: Color) -> Result<usize, Error> {
        match self.squares().iter().position(|&square| {
            square
                == Some(Piece {
                    kind: Kind::King,
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
    /// Only requires a mutable reference when the [`unmove`] algorithm is being used.
    ///
    /// # Errors
    ///
    /// This function will return an error if move generation fails.
    pub fn gen_and_display_moves(&mut self) {
        let moves = self.gen_legal_moves();
        self.display_moves(&moves, vec![]);
    }

    /// Move piece from [`start_index`] to [`target_index`.]
    ///
    /// # Errors
    ///
    /// This function will return an error if [`start_index`] or [`target_index`] are not within
    /// 0..64, if [`start_index`] is an empty square, or if the move is not legal.
    pub fn dbg_play_move(
        &mut self,
        start_index: usize,
        target_index: usize,
        promotion_kind: Option<Kind>,
    ) -> Result<(), Error> {
        if let Some(kind) = promotion_kind {
            self.make_move(&Move::new(start_index, target_index).set_promotion_kind(kind));
        } else {
            self.make_move(&Move::new(start_index, target_index));
        }
        Ok(())
    }

    /// Get the current number of legal moves.
    ///
    /// Only requires a mutable reference when the [`unmove`] algorithm is being used.
    ///
    /// # Errors
    ///
    /// This function will return an error if move generation fails.
    pub fn num_legal_moves(&mut self) -> Result<usize, Error> {
        Ok(self.gen_legal_moves().into_iter().collect::<Vec<_>>().len())
    }

    /// A debugging tool that displays the number of possible positions after [`depth`]
    /// for each legal move in the current position.
    ///
    /// Only requires a mutable reference when the [`unmove`] algorithm is being used.
    ///
    /// # Errors
    ///
    /// This function will return an error if move generation fails.
    ///
    /// See: [Perft](https://www.chessprogramming.org/Perft)
    pub fn perft(&mut self, depth: i32) -> Result<u32, Error> {
        if depth <= 0 {
            return Ok(0);
        }
        let moves = self.gen_legal_moves();

        let mut total_positions: u32 = 0;

        for mov in moves {
            let num_moves = match self.algorithm() {
                Algorithm::Clone => {
                    let mut board = self.clone();
                    board.make_move(&mov);
                    board.depth_num_positions(depth - 1)
                }
                Algorithm::Unmove => {
                    self.make_move(&mov);
                    let n = self.depth_num_positions(depth - 1);
                    self.unmake_move();
                    n
                }
            };
            total_positions += num_moves;
            println!("{mov}: {num_moves}");
        }
        println!("Total Positions: {total_positions}");
        Ok(total_positions)
    }

    /// Display the position history of the board in a human-readable format.
    /// History is only stored if the [`DEBUG_HISTORY`] global is set to true.
    pub fn display_history(&self) {
        for board in &self.history {
            for (i, square) in board.iter().enumerate() {
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

    /// Play a random game up to [`move_limit`] moves.
    /// Will leave the board in the last position of the game.
    pub fn play_random_game(&mut self, move_limit: u32) -> Result<GameState, Error> {
        use rand::{seq::SliceRandom, thread_rng};
        for _ in 0..move_limit {
            let moves = self.gen_legal_moves();

            let mut rng = thread_rng();
            let mov = moves.choose(&mut rng);
            // let mov = moves.into_iter().next();
            self.play_legal_move(mov);
            if self.state().game_state != GameState::Playing {
                break;
            }
        }
        Ok(self.state().game_state)
    }
}
