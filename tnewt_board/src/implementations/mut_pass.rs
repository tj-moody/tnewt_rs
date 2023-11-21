use color_eyre::eyre::Result;
use colored::Colorize;

use crate::board;
use board::Playable;

use crate::{piece, mov, color, castling, coordinate};

use piece::{Piece, PieceKind};
use mov::{BasicMove, CastlingMove, Move, PromotionMove};
use color::Color;
use coordinate::Coordinate;

// use crate::board::*;
use crate::magic_numbers::{
    SQUARES_TO_EDGE,
    DIRECTION_OFFSETS,
    KNIGHT_XY_OFFSETS,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Board {
    pub squares: [Option<Piece>; 64],
    pub state: board::State,
    pub state_history: Vec<board::State>,
    pub history: Vec<[Option<Piece>; 64]>,
    pub algorithm: board::Algorithm,
}

const DEBUG_HISTORY: bool = true;

// TODO: Add 0..64 range checking (basically everywhere)
impl Board {
    fn slider_gen_moves(
        &self,
        moves: &mut Vec<Move>,
        start_index: usize,
        piece: Piece,
    ) -> Result<(), board::Error> {
        let start_square = self.squares[start_index];

        let mut start_dir_index = 0;
        let mut end_dir_index = 8;
        let mut branch_length = 8;

        match piece.kind {
            PieceKind::Bishop => start_dir_index = 4,
            PieceKind::Rook => end_dir_index = 4,
            PieceKind::King => branch_length = 1,
            _ => (),
        }

        // for direction_index in start_dir_index..end_dir_index {
        for (direction_index, offset) in DIRECTION_OFFSETS
            .iter()
            .enumerate()
            .take(end_dir_index)
            .skip(start_dir_index) {
                for n in 0..branch_length.min(SQUARES_TO_EDGE[start_index][direction_index]) {
                    let target_index: usize = (start_index as i32
                    + (offset * (n + 1))) as usize;

                    let target_square = self.squares[target_index];

                    if Piece::is_same_color(start_square, target_square) { break }

                    moves.push(BasicMove {
                        start_index,
                        target_index,
                    }.into());

                    if target_square.is_some() { break }
                }
            }

        Ok(())
    }

    fn knight_gen_moves(
        &self,
        moves: &mut Vec<Move>,
        start_index: usize,
    ) {
        for offset in &KNIGHT_XY_OFFSETS {
            let (start_x, start_y) = (start_index % 8, start_index / 8 % 8);
            let (target_x, target_y) = (
                start_x as i32 + offset[0],
                start_y as i32 + offset[1],
            );
            if (0..8).contains(&target_x)
            && (0..8).contains(&target_y) {
                let target_index = (start_index as i32 + offset[2]) as usize;
                let start_square = self.squares[start_index];
                let target_square = self.squares[target_index];

                if !Piece::is_same_color(start_square, target_square) {
                    moves.push(BasicMove {
                        start_index,
                        target_index,
                    }.into());
                }
            }
        }

    }

    fn pawn_gen_moves(
        &self,
        moves: &mut Vec<Move>,
        start_index: usize,
    ) -> Result<(), board::Error> {
        let start_square = self.squares[start_index];

        let rank = start_index / 8 % 8;
        let file = start_index % 8;

        let color = match start_square {
            Some(piece) => piece.color,
            None => return Err(board::Error::MoveEmptySquare)
        };

        let offsets = match color {
            Color::White => [-8, -16, -9, -7],
            Color::Black => [ 8,  16,  7,  9],
        };
        let invalid_ranks = match color {
            Color::White => [0, 1],
            Color::Black => [7, 6],
        };
        let starting_rank = match color {
            Color::White => 6,
            Color::Black => 1,
        };
        if rank == invalid_ranks[0] { return Ok(()) }

        if rank != invalid_ranks[0] {
            let target_index: usize = (start_index as i32 + offsets[0]) as usize;
            let target_square = self.squares[target_index];
            if target_square.is_none() {
                let mov = BasicMove {
                    start_index,
                    target_index,
                };
                if rank == invalid_ranks[1] {
                    for promotion_move in &PromotionMove::from_move(mov) {
                        moves.push(Move::PromotionMove(*promotion_move));
                    }
                } else {
                    moves.push(mov.into());
                }
                if rank != invalid_ranks[1]
                && rank == starting_rank{
                    let target_index: usize = (start_index as i32 + offsets[1]) as usize;
                    let target_square = self.squares[target_index];
                    if target_square.is_none() {
                        moves.push(BasicMove {
                            start_index,
                            target_index,
                        }.into());
                    }
                }
            }
        }
        if file != 0 {
            let target_index: usize = (start_index as i32 + offsets[2]) as usize;
            let target_square = self.squares[target_index];
            if (!Piece::is_same_color(start_square, target_square)
            && !target_square.is_none())
            || Some(target_index) == self.state.ep_index
            {
                let mov = BasicMove {
                    start_index,
                    target_index,
                };
                if rank == invalid_ranks[1] {
                    for promotion_move in &PromotionMove::from_move(mov) {
                        moves.push(Move::PromotionMove(*promotion_move));
                    }
                } else {
                    moves.push(mov.into());
                }
            }
        }
        if file != 7 {
            let target_index: usize = (start_index as i32 + offsets[3]) as usize;
            let target_square = self.squares[target_index];
            if (!Piece::is_same_color(start_square, target_square)
            && !target_square.is_none())
            || Some(target_index) == self.state.ep_index
            {
                let mov = BasicMove {
                    start_index,
                    target_index,
                };
                if rank == invalid_ranks[1] {
                    for promotion_move in &PromotionMove::from_move(mov) {
                        moves.push(Move::PromotionMove(*promotion_move));
                    }
                } else {
                    moves.push(mov.into());
                }
            }
        }
        Ok(())
    }

    fn gen_castling_moves(&self, moves: &mut Vec<Move>) {
        self.state.castling_state.get_moves(&self.state.turn)
            .iter()
            .for_each(|&m| {
                let mut empty_indices: Vec<usize> = Vec::with_capacity(3);
                let mut king_index: usize = 64;
                let mut rook_index: usize = 64;

                if let Move::BasicMove(_) = m {
                    panic!("`CastlingState.get_moves()` returned `BasicMoves`s");
                } else if let Move::CastlingMove(castling_move) = m {
                    (empty_indices, _, king_index, rook_index) = castling_move.get_squares();
                }
                for &empty_index in &empty_indices {
                    match self.squares[empty_index] {
                        Some(_) => return,
                        None => (),
                    };
                }
                match self.squares[king_index] {
                    Some(piece) => match piece.kind {
                        PieceKind::King => if piece.color != self.state.turn { return },
                        _ => return,
                    },
                    None => return,
                };
                match self.squares[rook_index] {
                    Some(piece) => match piece.kind {
                        PieceKind::Rook => if piece.color != self.state.turn { return }
                        _ => return,
                    },
                    None => return,
                };
                moves.push(m);
            });
    }

    fn gen_pseudo_legal_moves(&self) -> Result<Vec<Move>, board::Error> {
        // TODO: Benchmark capacity past which performace gains diminish
        let mut moves: Vec<Move> = Vec::with_capacity(50);
        for start_index in 0..64 {
            let square = self.squares[start_index];
            if let Some(piece) = square {
                if piece.color != self.state.turn { continue }
                match piece.kind {
                    PieceKind::Bishop | PieceKind::Rook | PieceKind::Queen | PieceKind::King
                                      => self.slider_gen_moves(&mut moves, start_index, piece)?,
                    PieceKind::Knight => self.knight_gen_moves(&mut moves, start_index),
                    PieceKind::Pawn   => self.pawn_gen_moves(&mut moves, start_index)?,
                };
            }
        }
        self.gen_castling_moves(&mut moves);

        Ok(moves)
    }

    /// Only checks if the color whose turn it is has their piece at `index` being attacked
    fn attacked(&mut self, index: usize) -> Result<bool, board::Error> {
        let mut board = self.clone();

        // NOTE: Shouldn't be necessary to update board state with play_move,
        // as none of the board state can make possible/impossible a capture
        // on the next turn: en passant cannot capture a king, 50-move rule is
        // irrelevant, as checkmate supersedes it, castling rights are
        // irrelevant as castling cannot capture a piece

        // TODO: Convert to `self.change_turn()` and change_turn again at the end
        // of the function--currently results in white king and queenside rook being
        // mutated black in unmove algorithm for random games. Related to castling?
        // Turn on history and run unmove random game bench to debug. Doesn't create
        // bugs in num_moves tests, so probably move/unmove correctness-related.

        // self.change_turn();
        board.change_turn();

        let moves = match board.gen_pseudo_legal_moves() {
            Ok(moves) => moves,
            Err(e) => {
                // self.change_turn();
                board.change_turn();
                return Err(e);
            },
        };

        for mov in &moves {
            match mov {
                Move::BasicMove(m) => {
                    if m.target_index == index { return Ok(true); }
                },
                Move::PromotionMove(pm) => {
                    if pm.pawn_move.target_index == index { return Ok(true); }
                }
                // Cannot capture by castling
                Move::CastlingMove(_) => (),
            }
        }
        // self.change_turn();

        Ok(false)
    }

    fn play_optional_move(
        &mut self,
        m: Option<&Move>,
    ) -> Result<(), board::Error> {
        match m {
            Some(mov) => self.make_move(mov)?,
            None => {
                if self.attacked(self.king_index(self.state.turn)?)? {
                    self.state.game_state = board::GameState::Victory(
                        self.state.turn.opposite(),
                    );
                } else {
                    self.state.game_state = board::GameState::Draw;
                }
            }
        };
        Ok(())
    }

    /// This function does not actually mutate self, as it calls `make_move`
    /// and `unmake_move` sequentially, without mutating anywhere else.
    fn gen_legal_moves(&mut self) -> Result<Vec<Move>, board::Error> {
        Ok(self.gen_pseudo_legal_moves()?.into_iter().filter(|m| {
            let mut king_indices: Vec<usize>;
            if let Move::CastlingMove(cm) = m {
                king_indices = cm.get_squares().1;
                if self.attacked(self.king_index(self.state.turn).unwrap()).unwrap() {
                    return false;
                }
            } else {
                king_indices = vec![];
            }
            let opponent_responses: Vec<Move>;
            let king_index: usize;

            match self.algorithm {
                board::Algorithm::Clone => {
                    let mut board = self.clone();
                    board.make_move(m).unwrap();

                    opponent_responses = board.gen_pseudo_legal_moves().unwrap();
                    king_index = board.king_index(board.state.turn.opposite()).unwrap();

                },
                board::Algorithm::Unmove => {
                    self.make_move(m).unwrap();

                    opponent_responses = self.gen_pseudo_legal_moves().unwrap();
                    // king_index = self.king_index(self.state.turn.opposite()).unwrap();
                    king_index = match self.king_index(self.state.turn.opposite()) {
                        Ok(i) => i,
                        Err(_) => {
                            self.dbg_history();
                            self.display();
                            panic!();
                        },
                    };

                    self.unmake_move().unwrap();
                },
            }
            king_indices.push(king_index);

            for &response in &opponent_responses {
                match response {
                    Move::BasicMove(response) => {
                        if king_indices.contains(&response.target_index) { return false; }
                    },
                    Move::PromotionMove(response) => {
                        if king_indices.contains(&response.pawn_move.target_index) { return false; }
                    },
                    Move::CastlingMove(_) => (),
                }
            }

            true
        }).collect::<Vec<Move>>())
    }
}

impl Playable for Board {
    fn from_chars(chars: &[char; 64]) -> Result<Self, board::Error> {
        let mut squares: [Option<Piece>; 64] = [None; 64];
        for (i, piece_char) in chars.iter().enumerate() {
            let piece = match piece_char.to_ascii_lowercase() {
                'k' => Some(PieceKind::King),
                'q' => Some(PieceKind::Queen),
                'r' => Some(PieceKind::Rook),
                'b' => Some(PieceKind::Bishop),
                'n' => Some(PieceKind::Knight),
                'p' => Some(PieceKind::Pawn),
                ' ' => None,
                _ => return Err(board::Error::InvalidPieceChar(*piece_char))
            };
            squares[i] = match piece_char {
                'k' | 'q' | 'r' | 'b' | 'n' | 'p' => Piece {
                    kind: piece.unwrap(),
                    color: Color::Black
                }.into(),
                'K' | 'Q' | 'R' | 'B' | 'N' | 'P' => Piece {
                    kind: piece.unwrap(),
                    color: Color::White
                }.into(),
                ' ' => None,
                _ => return Err(board::Error::InvalidPieceChar(*piece_char)),
            };
        }
        Ok(Board {
            squares,
            state: board::State {
                turn: Color::White,
                castling_state: castling::State {
                    white: castling::Rights::Both,
                    black: castling::Rights::Both,
                },
                ep_index: None,
                halfmove_clock: 0,
                fullmove_count: 1,
                game_state: board::GameState::Playing,
                last_captured_square: None,
                last_move: None,
                last_ep_taken_index: None,
            },
            state_history: vec![],
            history: vec![],
            algorithm: board::Algorithm::Clone,
        })
    }
    fn new() -> Self {
        Board::from_chars(&[
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

    fn display(&self) {
        for (i, &square) in self.squares.iter().enumerate() {
            print!("{} ", Piece::display_square(square));
            if i % 8 == 7 {
                for j in i - 7..=i {
                    print!(" {}", format!("{j: >2}").black());
                }
                println!();
            }
        }
    }

    fn dbg_moves(
        &self,
        moves: &[Move],
        shown_pieces: Vec<PieceKind>,
        show_castling: bool,
    ) -> Result<(), board::Error> {
        BasicMove::dbg_moves(&moves
            .iter()
            .filter_map(|m| match m {
                Move::BasicMove(bm) => Some(*bm),
                _ => None
            })
            .map(|m| {
                if shown_pieces == &[] { return Ok(Some(m)); }
                let piece_kind = match self.squares[m.start_index] {
                    Some(piece) => piece.kind,
                    None => return Err(board::Error::MoveEmptySquare),
                };
                if shown_pieces.contains(&piece_kind) {
                    return Ok(Some(m));
                }
                Ok(None)
            })
            .collect::<Result<Vec<Option<_>>, board::Error>>()?
            .into_iter()
            .filter_map(|m| m)
            .collect::<Vec<_>>(),
            self
        );
        if !show_castling { return Ok(()) }
        CastlingMove::dbg_moves(&moves
            .iter()
            .filter_map(|m| match m {
                Move::CastlingMove(cm) => Some(*cm), _ => None
            })
            .collect::<Vec<_>>()
        );
        PromotionMove::dbg_moves(&moves
            .iter()
            .filter_map(|m| match m{
                Move::PromotionMove(pm) => Some(*pm),
                _ => None
            })
            .collect::<Vec<_>>(),
            self
        );
        Ok(())
    }

    fn from_fen(fen: &str) -> Result<Self, board::Error> {
        // TODO: Handle errors
        let mut squares: Vec<char> = vec![];
        let mut iter = fen.split(' ');
        let pieces            = iter.next().expect("Invalid FEN");
        let turn              = iter.next().expect("Invalid FEN");
        let castling_rights   = iter.next().expect("Invalid FEN");
        let ep_square         = iter.next().expect("Invalid FEN");
        let halfmove_clock    = iter.next().expect("Invalid FEN");
        let fullmove_count    = iter.next().expect("Invalid FEN");
        for row in pieces.split('/') {
            for c in row.chars() {
                match c {
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                        squares.append(
                            &mut vec![' '; c.to_digit(10).unwrap() as usize]
                        );
                    }
                    'k' | 'q' | 'r' | 'b' | 'n' | 'p' |
                        'K' | 'Q' | 'R' | 'B' | 'N' | 'P' => {
                            squares.append(&mut vec![c]);
                        }
                    _ => return Err(board::Error::InvalidPieceChar(c)),
                }
            }
        }

        let array: [char; 64] = match squares.try_into() {
            Ok(some) => some,
            Err(_) => return Err(board::Error::InvalidFenBoardLength(fen.to_string())),
        };

        let mut board: Board = Board::from_chars(&array)?;
        board.state.turn = Color::from(turn);
        board.state.castling_state = castling::State::from(castling_rights);
        board.state.ep_index = Coordinate::from(ep_square)?
            .map(Coordinate::into_index);
        board.state.halfmove_clock = match halfmove_clock.parse::<u32>() {
            Ok(v) => v,
            Err(e) => return Err(board::Error::InvalidHalfmoveStr(e)),
        };
        board.state.fullmove_count = match fullmove_count.parse::<u32>() {
            Ok(v) => v,
            Err(e) => return Err(board::Error::InvalidFullmoveStr(e)),
        };
        Ok(board)
    }

    fn to_fen(&self) -> &str {
        let mut squares = [' '; 64];
        for (i, &square) in self.squares.iter().enumerate() {
            squares[i] = Piece::square_to_char(square);
        }
        for char in squares.iter() {
            let _ = char; todo!()
        }
        todo!()
    }


    fn change_turn(&mut self) {
        self.state.turn = self.state.turn.opposite();
    }


    fn make_move(&mut self, m: &Move) -> Result<(), board::Error> {
        if self.algorithm == board::Algorithm::Unmove {
            self.state_history.push(self.state);
        }
        let color = self.state.turn;
        let mov: BasicMove = match m {
            Move::BasicMove(mov) => *mov,
            Move::CastlingMove(mov) => {
                self.state.castling_state.revoke(castling::Rights::Both, &color);
                mov.king_move
            },
            Move::PromotionMove(mov) => mov.pawn_move,
        };

        let ep_taken_index: Option<usize> = (|| {
            if let Some(index) = self.state.ep_index {
                if mov.target_index != index { return None }
                if let Some(piece) = self.squares[mov.start_index] {
                    if piece.kind != PieceKind::Pawn { return None }
                    return Some(match color {
                        Color::White => mov.target_index + 8,
                        Color::Black => mov.target_index - 8,
                    });
                }
            }
            None
        })();

        let moving_piece = self.squares[mov.start_index];
        let captured_piece = self.squares[mov.target_index];

        // TODO: Always store these parameters for unmoving to always be possible?
        if self.algorithm == board::Algorithm::Unmove {
            self.state.last_ep_taken_index = ep_taken_index;
            self.state.last_move = Some(*m);
            self.state.last_captured_square = Some(self.squares[mov.target_index]);
        }

        self.state.ep_index = None;
        match captured_piece {
            Some(piece) => {
                match piece.kind {
                    PieceKind::King => {
                        // println!("{:?} moved into check", color.opposite());
                        // NOTE: Could be an error in some cases, but necessary
                        // to allow to check if currently in check
                    },
                    PieceKind::Rook => {
                        match mov.target_index {
                            0 | 56 => self.state.castling_state.revoke(
                                castling::Rights::Queenside,
                                &color.opposite(),
                            ),
                            7 | 63 => self.state.castling_state.revoke(
                                castling::Rights::Kingside,
                                &color.opposite(),
                            ),
                            _  => (),
                        };
                    },
                    _ => (),
                };
                self.state.halfmove_clock = 0;
            },
            None => {
                if ep_taken_index.is_none() {
                    self.state.halfmove_clock += 1;
                }
            },
        };

        match moving_piece {
            Some(piece) => match piece.kind {
                PieceKind::King => self.state.castling_state.revoke(
                    castling::Rights::Both,
                    &self.state.turn,
                ),
                PieceKind::Rook => match mov.start_index {
                    0 | 56 => self.state.castling_state
                        .revoke(castling::Rights::Queenside, &color),
                    7 | 63 => self.state.castling_state
                        .revoke(castling::Rights::Kingside,  &color),
                    _  => (),
                }
                PieceKind::Pawn => {
                    self.state.halfmove_clock = 0;
                    let offset: isize = mov.target_index as isize
                    - mov.start_index as isize;
                    self.state.ep_index = match offset {
                        -16 => Some(mov.start_index - 8),
                        16  => Some(mov.start_index + 8),
                        _   => None,
                    };
                }
                _ => (),
            },
            None => {
                return Err(board::Error::MoveEmptySquare)
            },

        };
        if self.state.halfmove_clock >= 50  {
            self.state.game_state = board::GameState::Draw;
        }

        if DEBUG_HISTORY {
            self.history.push(self.squares);
        }

        match m {
            Move::BasicMove(m) => {
                if let Some(pawn_index) = ep_taken_index {
                    self.squares[pawn_index] = None;
                }
                self.squares[m.target_index] = self.squares[m.start_index];
                self.squares[m.start_index] = None;
            },
            Move::CastlingMove(cm) => {
                self.squares[cm.king_move.target_index] = Some(Piece {
                    kind: PieceKind::King,
                    color,
                });
                self.squares[cm.rook_move.target_index] = Some(Piece {
                    kind: PieceKind::Rook,
                    color,
                });
                self.squares[cm.king_move.start_index] = None;
                self.squares[cm.rook_move.start_index] = None;
            }
            Move::PromotionMove(pm) => {
                self.squares[pm.pawn_move.target_index] = Some(Piece {
                    kind: pm.promotion_kind,
                    color,
                });
                self.squares[pm.pawn_move.start_index] = None;
            }
        }

        if self.state.turn == Color::Black { self.state.fullmove_count += 1; }
        self.change_turn();

        Ok(())
    }

    fn unmake_move(&mut self) -> Result<(), board::Error> {
        let last_captured_square = match self.state.last_captured_square {
            Some(square) => square,
            None => return Err(board::Error::UndoFromFirstMove),
        };
        let last_move = match self.state.last_move {
            Some(mov) => mov,
            None => return Err(board::Error::UndoFromFirstMove),
        };
        let last_ep_taken_index = self.state.last_ep_taken_index;

        self.state = match self.state_history.pop() {
            Some(state) => state,
            None => return Err(board::Error::UndoFromFirstMove),
        };

        // Color of the player who made the move being undone
        let color = self.state.turn;

        if DEBUG_HISTORY {
            self.history.pop();
        }

        match last_move {
            Move::BasicMove(mov) => {
                if let Some(index) = last_ep_taken_index {
                    self.squares[index] = Some(Piece {
                        kind: PieceKind::Pawn,
                        color: self.state.turn.opposite(),
                    });
                }
                let last_moved_square = self.squares[mov.target_index];
                self.squares[mov.target_index] = last_captured_square;
                self.squares[mov.start_index] = last_moved_square;
            },
            Move::CastlingMove(cm) => {
                self.squares[cm.king_move.start_index] = Some(Piece {
                    kind: PieceKind::King,
                    color,
                });
                self.squares[cm.rook_move.start_index] = Some(Piece {
                    kind: PieceKind::Rook,
                    color,
                });
                self.squares[cm.king_move.target_index] = None;
                self.squares[cm.rook_move.target_index] = None;
            },
            Move::PromotionMove(pm) => {
                self.squares[pm.pawn_move.target_index] = last_captured_square;
                self.squares[pm.pawn_move.start_index] = Some(Piece {
                    kind: PieceKind::Pawn,
                    color,
                });
            },
        }

        Ok(())
    }

    fn king_index(&self, color: Color) -> Result<usize, board::Error> {
        match self.squares
            .iter()
            .position(|&square| square == Some(Piece {
                kind: PieceKind::King, color,
            })) {
                Some(index) => Ok(index),
                None => { Err(board::Error::NoKing) },
            }
    }

    fn dbg_set_algorithm(&mut self, algorithm: board::Algorithm) {
        self.algorithm = algorithm;
    }

    fn dbg_play_move(
        &mut self,
        start_index: usize,
        target_index: usize,
    ) -> Result<(), board::Error> {
        self.make_move(&BasicMove::from(&[start_index, target_index]).into())?;
        Ok(())
    }

    fn dbg_no_castle(&mut self) {
        self.state.castling_state.revoke(castling::Rights::Both, &Color::White);
        self.state.castling_state.revoke(castling::Rights::Both, &Color::Black);
    }

    fn dbg_move_count(&mut self) -> Result<usize, board::Error> {
        Ok(self.gen_legal_moves()?.len())
    }

    fn dbg_depth_num_positions(&mut self, depth: i32) -> Result<u32, board::Error> {
        if depth <= 0 { return Ok(1) }
        let moves = self.gen_legal_moves()?;
        let mut num_positions: u32 = 0;

        match self.algorithm {
            board::Algorithm::Clone => {
                for mov in &moves {
                    let mut board = self.clone();
                    board.make_move(mov)?;
                    num_positions += board.dbg_depth_num_positions(depth - 1)?;
                }
            },
            board::Algorithm::Unmove => {
                for mov in &moves {
                    self.make_move(mov)?;
                    num_positions += self.dbg_depth_num_positions(depth - 1)?;
                    self.unmake_move()?;
                }
            },
        }

        Ok(num_positions)
    }

    fn perft(&mut self, depth: i32) -> Result<u32, board::Error> {
        if depth <= 0 {
            return Ok(0);
        }
        let moves = self.gen_legal_moves()?;

        let mut total_positions: u32 = 0;

        for mov in &moves {
            let num_moves = match self.algorithm {
                board::Algorithm::Clone => {
                    let mut board = self.clone();
                    board.make_move(mov)?;
                    board.dbg_depth_num_positions(depth - 1)?

                },
                board::Algorithm::Unmove => {
                    self.make_move(mov)?;
                    let n = self.dbg_depth_num_positions(depth - 1)?;
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

    fn dbg_history(&self) {
        for board in &self.history {
            for (i, &square) in board.iter().enumerate() {
                print!("{} ", Piece::display_square(square));
                if i % 8 == 7 {
                    for j in i - 7..=i {
                        print!(" {}", format!("{j: >2}").black());
                    }
                    println!();
                }
            }
            println!("{:?}", self.state.turn);
        }
    }
    fn play_random_game(
        &mut self,
        move_limit: u32,
    ) -> Result<board::GameState, board::Error> {
        use rand::{seq::SliceRandom, thread_rng};
        for _ in 0..move_limit {
            let moves = self.gen_legal_moves()?;

            let mut rng = thread_rng();
            let mov = moves.choose(&mut rng);
            self.play_optional_move(mov)?;
            if self.state.game_state != board::GameState::Playing { break }
        }
        Ok(self.state.game_state)
    }

    fn dbg_gen_moves(&mut self) -> Result<(), board::Error> {
        let moves = self.gen_legal_moves()?;
        self.dbg_moves(&moves, vec![], true)?;
        Ok(())
    }

    fn squares(&self) -> [Option<Piece>; 64] {
        self.squares
    }

    fn state(&self) -> board::State {
        self.state
    }
}
