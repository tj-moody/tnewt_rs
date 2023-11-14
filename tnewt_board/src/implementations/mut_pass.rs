#![allow(dead_code)]
use color_eyre::eyre::Result;
use colored::Colorize;

use crate::board::{
    PlayableBoard,
    GameState,
    BoardState,
    Algorithm,
};

use crate::square::*;
use crate::mov::*;
use crate::color::*;
use crate::castling::*;
use crate::coordinate::*;

// use crate::board::*;
use crate::magic_numbers::{
    SQUARES_TO_EDGE,
    DIRECTION_OFFSETS,
    KNIGHT_XY_OFFSETS,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Board {
    pub squares: [Square; 64],
    pub state: BoardState,
    pub state_history: Vec<BoardState>,
    pub history: Vec<[Square; 64]>,
    pub algorithm: Algorithm,
}

impl Board {
    fn slider_gen_moves(&self, moves: &mut Vec<Move>, start_index: usize) -> Result<(), String> {
        let square = self.squares[start_index];

        let mut start_dir_index = 0;
        let mut end_dir_index = 8;
        let mut branch_length = 8;

        let piece = square.piece()?;
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

                    let target_piece = target_square.piece();
                    if let Ok(target_piece) = target_piece {
                        if target_piece.color == piece.color { break }
                    }

                    moves.push(BasicMove {
                        start_index,
                        target_index,
                    }.into());

                    if !target_square.is_empty() { break }
                }
            }

        Ok(())
    }

    fn knight_gen_moves(&self, moves: &mut Vec<Move>, start_index: usize) -> Result<(), String> {
        for offset in KNIGHT_XY_OFFSETS.iter() {
            let (start_x, start_y) = (start_index % 8, start_index / 8 % 8);
            let (target_x, target_y) = (
                start_x as i32 + offset[0],
                start_y as i32 + offset[1],
            );
            if (0..8).contains(&target_x)
            && (0..8).contains(&target_y) {
                let target_index = (start_index as i32 + offset[2]) as usize;
                let square = self.squares[start_index];
                let target_square = self.squares[target_index];

                if !square.is_same_color(target_square) {
                    moves.push(BasicMove {
                        start_index,
                        target_index,
                    }.into());
                }
            }
        }
        Ok(())
    }

    fn pawn_gen_moves(&self, moves: &mut Vec<Move>, start_index: usize) -> Result<(), String> {
        let square = self.squares[start_index];

        let rank = start_index / 8 % 8;
        let file = start_index % 8;

        let color = square.piece()?.color;

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
            if target_square.is_empty() {
                let mov = BasicMove {
                    start_index,
                    target_index,
                };
                if rank == invalid_ranks[1] {
                    for promotion_move in PromotionMove::from_move(mov).iter() {
                        moves.push(Move::PromotionMove(*promotion_move))
                    }
                } else {
                    moves.push(mov.into());
                }
                if rank != invalid_ranks[1]
                && rank == starting_rank{
                    let target_index: usize = (start_index as i32 + offsets[1]) as usize;
                    let target_square = self.squares[target_index];
                    if target_square.is_empty() {
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
            if (!target_square.is_same_color(square)
            && !target_square.is_empty())
            || Some(target_index) == self.state.ep_index
            {
                let mov = BasicMove {
                    start_index,
                    target_index,
                };
                if rank == invalid_ranks[1] {
                    for promotion_move in PromotionMove::from_move(mov).iter() {
                        moves.push(Move::PromotionMove(*promotion_move))
                    }
                } else {
                    moves.push(mov.into());
                }
            }
        }
        if file != 7 {
            let target_index: usize = (start_index as i32 + offsets[3]) as usize;
            let target_square = self.squares[target_index];
            if (!target_square.is_same_color(square)
            && !target_square.is_empty())
            || Some(target_index) == self.state.ep_index
            {
                let mov = BasicMove {
                    start_index,
                    target_index,
                };
                if rank == invalid_ranks[1] {
                    for promotion_move in PromotionMove::from_move(mov).iter() {
                        moves.push(Move::PromotionMove(*promotion_move))
                    }
                } else {
                    moves.push(mov.into());
                }
            }
        }
        Ok(())
    }

    fn gen_castling_moves(&self, moves: &mut Vec<Move>)  -> Result<(), String> {
        self.state.castling_state.get_moves(&self.state.turn)?
            .iter()
            .for_each(|&m| {
                // let mut empty_indices: Vec<usize> = vec![];
                let mut empty_indices: Vec<usize> = vec![];
                let mut king_index: usize = 64;
                let mut rook_index: usize = 64;

                if let Move::BasicMove(_) = m {
                    panic!("`CastlingState.get_moves()` returned `BasicMoves`s");
                } else if let Move::CastlingMove(castling_move) = m {
                    (empty_indices, _, king_index, rook_index) = castling_move.get_squares();
                }
                for &empty_index in empty_indices.iter() {
                    match self.squares[empty_index] {
                        Square::Some(_) => return,
                        Square::Empty => (),
                    };
                }
                match self.squares[king_index] {
                    Square::Some(piece) => match piece.kind {
                        PieceKind::King => if piece.color != self.state.turn { return },
                        _ => return,
                    },
                    Square::Empty => return,
                };
                match self.squares[rook_index] {
                    Square::Some(piece) => match piece.kind {
                        PieceKind::Rook => if piece.color != self.state.turn { return }
                        _ => return,
                    },
                    Square::Empty => return,
                };
                moves.push(m);
            });
        Ok(())
    }

    fn gen_pseudo_legal_moves(&self) -> Result<Vec<Move>, String> {
        // TODO: Benchmark capacity past which performace gains diminish
        let mut moves: Vec<Move> = Vec::with_capacity(50);
        for start_index in 0..64 {
            let square = self.squares[start_index];
            if let Square::Some(piece) = square {
                if piece.color != self.state.turn { continue }
                match piece.kind {
                    PieceKind::Bishop | PieceKind::Rook | PieceKind::Queen | PieceKind::King
                                      => self.slider_gen_moves(&mut moves, start_index)?,
                    PieceKind::Knight => self.knight_gen_moves(&mut moves, start_index)?,
                    PieceKind::Pawn   => self.pawn_gen_moves(&mut moves, start_index)?,
                };
            }
        }
        self.gen_castling_moves(&mut moves)?;

        Ok(moves)
    }

    /// Only checks if the color whose turn it is is in check
    fn attacked(&self, index: usize) -> Result<bool, String> {
        let mut board = self.clone();
        board.change_turn();

        // NOTE: Shouldn't be necessary to update board state with play_move,
        // as none of the board state can make possible/impossible a capture
        // on the next turn: en passant cannot capture a king, 50-move rule is
        // irrelevant, as checkmate supersedes it, castling rights are
        // irrelevant as castling cannot capture a piece

        let moves = board.gen_pseudo_legal_moves()?;

        for mov in moves.iter() {
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

        Ok(false)
    }

    fn play_optional_move(
        &mut self,
        m: Option<&Move>,
    ) -> Result<(), String> {
        match m {
            Some(mov) => self.make_move(mov)?,
            None => {
                if self.attacked(self.king_index(self.state.turn)?)? {
                    self.state.game_state = GameState::Victory(
                        self.state.turn.opposite(),
                    );
                } else {
                    self.state.game_state = GameState::Draw;
                }
            }
        };
        Ok(())
    }
}

impl PlayableBoard for Board {
    fn from_chars(chars: &[char; 64]) -> std::result::Result<Self, String> {
        let mut squares = [Square::Empty; 64];
        for (i, piece_char) in chars.iter().enumerate() {
            let piece = match piece_char.to_ascii_lowercase() {
                'k' => Some(PieceKind::King),
                'q' => Some(PieceKind::Queen),
                'r' => Some(PieceKind::Rook),
                'b' => Some(PieceKind::Bishop),
                'n' => Some(PieceKind::Knight),
                'p' => Some(PieceKind::Pawn),
                ' ' => None,
                _ => None, // TODO: Handle error
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
                ' ' => Square::Empty,
                _ => return Err(format!("Invalid char: `{}`", piece_char)),
            };
        }
        Ok(Board {
            squares,
            state: BoardState {
                turn: Color::White,
                castling_state: CastlingState {
                    white: CastlingRights::Both,
                    black: CastlingRights::Both,
                },
                ep_index: None,
                halfmove_clock: 0,
                fullmove_count: 1,
                game_state: GameState::Playing,
                last_captured_square: None,
                last_move: None,
                last_ep_taken_index: None,
            },
            state_history: vec![],
            history: vec![],
            algorithm: Algorithm::Clone,
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
        for (i, square) in self.squares.iter().enumerate() {
            print!("{} ", square.display());
            if i % 8 == 7 {
                for j in i - 7..=i {
                    print!(" {}", format!("{: >2}", j).black());
                }
                println!();
            }
        }
    }

    fn dbg_moves(
        &self,
        moves: &Vec<Move>,
        shown_pieces: Vec<PieceKind>,
        show_castling: bool,
    ) -> Result<(), String> {
        BasicMove::dbg_moves(&moves
            .iter()
            .filter_map(|m| match m {
                Move::BasicMove(bm) => Some(*bm),
                _ => None
            })
            .filter(|m| {
                shown_pieces == vec![] ||
                shown_pieces.contains(&self.squares[m.start_index]
                    .piece()
                    .unwrap()
                    .kind
                )
            })
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

    fn from_fen(fen: &str) -> Result<Self, String> {
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
                        )
                    }
                    'k' | 'q' | 'r' | 'b' | 'n' | 'p' |
                        'K' | 'Q' | 'R' | 'B' | 'N' | 'P' => {
                            squares.append(&mut vec![c])
                        }
                    _ => return Err(format!("Invalid FEN (char): {}", fen)),
                }
            }
        }

        let array: [char; 64] = match squares.try_into() {
            Ok(some) => some,
            Err(_) => return Err(format!("Invalid FEN (len): {}", fen)),
        };

        let mut board: Board = Board::from_chars(&array)?;
        board.state.turn = Color::from(turn);
        board.state.castling_state = CastlingState::from(castling_rights);
        board.state.ep_index = Coordinate::from(ep_square)?
            .map(|coordinate| coordinate.into_index());
        board.state.halfmove_clock = halfmove_clock.parse::<u32>().unwrap();
        board.state.fullmove_count = fullmove_count.parse::<u32>().unwrap();
        Ok(board)
    }

    fn to_fen(&self) -> &str {
        let mut squares = [' '; 64];
        for (i, square) in self.squares.iter().enumerate() {
            squares[i] = square.to_char()
        }
        for char in squares.iter() {
            let _ = char; todo!()
        }
        todo!()
    }


    fn change_turn(&mut self) {
        self.state.turn = self.state.turn.opposite();
    }

    /// This function does not actually mutate self, as it calls `make_move`
    /// and `unmake_move` sequentially, without mutating anywhere else.
    fn gen_legal_moves(&mut self) -> Result<Vec<Move>, String> {
        Ok(self.gen_pseudo_legal_moves()?.into_iter().filter(|m| {
            let mut king_indices: Vec<usize>;
            if let Move::CastlingMove(cm) = m {
                king_indices = cm.get_squares().1.to_vec();
                if self.attacked(self.king_index(self.state.turn).unwrap()).unwrap() {
                    return false;
                }
            } else {
                king_indices = vec![];
            }
            let opponent_responses: Vec<Move>;
            let king_index: usize;

            // PERF: Cloning allows maintaining shared reference, but results
            // in a ~250% reduction in speed to play a full random game
            match self.algorithm {
                Algorithm::Clone => {
                    let mut board = self.clone();
                    board.make_move(m).unwrap();

                    opponent_responses = board.gen_pseudo_legal_moves().unwrap();
                    king_index = board.king_index(board.state.turn.opposite()).unwrap();

                },
                Algorithm::Unmove => {
                    self.make_move(m).unwrap();

                    opponent_responses = self.gen_pseudo_legal_moves().unwrap();
                    king_index = self.king_index(self.state.turn.opposite()).unwrap();

                    self.unmake_move().unwrap();
                },
            }
            king_indices.push(king_index);

            for &response in opponent_responses.iter() {
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

    fn make_move(&mut self, m: &Move) -> Result<(), String> {
        if self.algorithm == Algorithm::Unmove {
            self.state_history.push(self.state);
        }
        let color = self.state.turn;
        let mov: BasicMove = match m {
            Move::BasicMove(mov) => *mov,
            Move::CastlingMove(mov) => {
                self.state.castling_state.revoke(CastlingRights::Both, &color);
                mov.king_move
            },
            Move::PromotionMove(mov) => mov.pawn_move,
        };

        let ep_taken_index: Option<usize> = (|| {
            if let Some(index) = self.state.ep_index {
                if mov.target_index != index { return None }
                if let Square::Some(piece) = self.squares[mov.start_index] {
                    if piece.kind != PieceKind::Pawn { return None }
                    return Some((match color {
                        Color::White => mov.target_index + 8,
                        Color::Black => mov.target_index - 8,
                    }) as usize);
                }
            }
            None
        })();

        let moving_piece = self.squares[mov.start_index];
        let captured_piece = self.squares[mov.target_index];

        if self.algorithm == Algorithm::Unmove {
            self.state.last_ep_taken_index = ep_taken_index;
            self.state.last_move = Some(*m);
            self.state.last_captured_square = Some(self.squares[mov.target_index]);
        }

        self.state.ep_index = None;
        match captured_piece {
            Square::Some(piece) => {
                match piece.kind {
                    PieceKind::King => {
                        // println!("{:?} moved into check", color.opposite());
                        // NOTE: Could be an error in some cases, but necessary
                        // to allow to check if currently in check
                    },
                    PieceKind::Rook => {
                        match mov.target_index {
                            0 | 56 => self.state.castling_state.revoke(
                                CastlingRights::Queenside,
                                &color.opposite(),
                            ),
                            7 | 63 => self.state.castling_state.revoke(
                                CastlingRights::Kingside,
                                &color.opposite(),
                            ),
                            _  => (),
                        };
                    },
                    _ => (),
                };
                self.state.halfmove_clock = 0;
            },
            Square::Empty => {
                if ep_taken_index.is_none() {
                    self.state.halfmove_clock += 1
                }
            },
        };

        match moving_piece {
            Square::Some(piece) => match piece.kind {
                PieceKind::King => self.state.castling_state.revoke(
                    CastlingRights::Both,
                    &self.state.turn,
                ),
                PieceKind::Rook => match mov.start_index {
                    0 | 56 => self.state.castling_state
                        .revoke(CastlingRights::Queenside, &color),
                    7 | 63 => self.state.castling_state
                        .revoke(CastlingRights::Kingside,  &color),
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
            Square::Empty => {
                return Err(format!("Attempt to move empty square {:?}", m))
            },

        };
        if self.state.halfmove_clock >= 50  {
            self.state.game_state = GameState::Draw
        }

        // if self.algorithm == Algorithm::Unmove {
        //     self.history.push(self.squares);
        // }
        match m {
            Move::BasicMove(m) => {
                if let Some(pawn_index) = ep_taken_index {
                    self.squares[pawn_index as usize] = Square::Empty;
                }
                self.squares[m.target_index] = self.squares[m.start_index];
                self.squares[m.start_index] = Square::Empty;
            },
            Move::CastlingMove(cm) => {
                self.squares[cm.king_move.target_index] = Square::Some(Piece{
                    kind: PieceKind::King,
                    color,
                });
                self.squares[cm.rook_move.target_index] = Square::Some(Piece{
                    kind: PieceKind::Rook,
                    color,
                });
                self.squares[cm.king_move.start_index] = Square::Empty;
                self.squares[cm.rook_move.start_index] = Square::Empty;
            }
            Move::PromotionMove(pm) => {
                self.squares[pm.pawn_move.target_index] = Square::Some(Piece { kind: pm.promotion_kind, color });
                self.squares[pm.pawn_move.start_index] = Square::Empty;
            }
        }

        if self.state.turn == Color::Black { self.state.fullmove_count += 1; }
        self.change_turn();

        Ok(())
    }

    fn unmake_move(&mut self) -> Result<(), String> {
        let last_captured_square = self.state.last_captured_square
            .expect("Attempt to undo from first move");
        let last_move = self.state.last_move
            .expect("Attempt to undo from first move");
        let last_ep_taken_index = self.state.last_ep_taken_index;

        self.state = self.state_history.pop()
            .expect("Attempt to undo from first move");

        // Color of the player who made the move being undone
        let color = self.state.turn;

        self.history.pop();
        match last_move {
            Move::BasicMove(mov) => {
                if let Some(index) = last_ep_taken_index {
                    self.squares[index] = Square::Some(Piece {
                        kind: PieceKind::Pawn,
                        color: self.state.turn.opposite(),
                    });
                }
                let last_moved_square = self.squares[mov.target_index];
                self.squares[mov.target_index] = last_captured_square;
                self.squares[mov.start_index] = last_moved_square;
            },
            Move::CastlingMove(cm) => {
                self.squares[cm.king_move.start_index] = Square::Some(Piece {
                    kind: PieceKind::King,
                    color,
                });
                self.squares[cm.rook_move.start_index] = Square::Some(Piece {
                    kind: PieceKind::Rook,
                    color,
                });
                self.squares[cm.king_move.target_index] = Square::Empty;
                self.squares[cm.rook_move.target_index] = Square::Empty;
            },
            Move::PromotionMove(pm) => {
                self.squares[pm.pawn_move.target_index] = last_captured_square;
                self.squares[pm.pawn_move.start_index] = Square::Some(Piece {
                    kind: PieceKind::Pawn,
                    color,
                });
            },
        }

        Ok(())
    }

    fn king_index(&self, color: Color) -> Result<usize, String> {
        match self.squares
            .iter()
            .position(|&square| square == Square::Some(Piece {
                kind: PieceKind::King, color,
            })) {
                Some(index) => Ok(index),
                None => { Err("Current player has no king!".to_string()) },
            }
    }

    fn dbg_set_algorithm(&mut self, algorithm: Algorithm) {
        self.algorithm = algorithm;
    }

    fn dbg_play_move(&mut self, m: &[usize; 2]) -> Result<(), String> {
        self.make_move(&BasicMove::from(m).into())?;
        Ok(())
    }

    fn dbg_no_castle(&mut self) {
        self.state.castling_state.revoke(CastlingRights::Both, &Color::White);
        self.state.castling_state.revoke(CastlingRights::Both, &Color::Black);
    }

    fn dbg_move_count(&mut self) -> Result<u32, String> {
        Ok(self.gen_legal_moves()?.len() as u32)
    }

    fn dbg_depth_num_positions(&mut self, depth: i32) -> Result<u32, String> {
        if depth <= 0 { return Ok(1) }
        let moves = self.gen_legal_moves()?;
        let mut num_positions: u32 = 0;

        match self.algorithm {
            Algorithm::Clone => {
                for mov in moves.iter() {
                    let mut board = self.clone();
                    board.make_move(mov)?;
                    num_positions += board.dbg_depth_num_positions(depth - 1)?;
                }
            },
            Algorithm::Unmove => {
                for mov in moves.iter() {
                    self.make_move(mov)?;
                    num_positions += self.dbg_depth_num_positions(depth - 1)?;
                    self.unmake_move()?;
                }
            },
        }

        Ok(num_positions)
    }

    fn perft(&mut self, depth: i32) -> Result<u32, String> {
        if depth <= 0 {
            return Ok(0);
        }
        let moves = self.gen_legal_moves()?;

        let mut total_positions: u32 = 0;

        for mov in moves.iter() {
            let num_moves = match self.algorithm {
                Algorithm::Clone => {
                    let mut board = self.clone();
                    board.make_move(mov)?;
                    board.dbg_depth_num_positions(depth - 1)?

                },
                Algorithm::Unmove => {
                    self.make_move(mov)?;
                    let n = self.dbg_depth_num_positions(depth - 1)?;
                    self.unmake_move()?;
                    n
                },
            };
            total_positions += num_moves;
            println!("{}: {}", mov.to_string(), num_moves);
        }
        println!("Total Positions: {}", total_positions);
        Ok(total_positions)
    }

    fn dbg_history(&self) {
        for board in self.history.iter() {
            for (i, square) in board.iter().enumerate() {
                print!("{} ", square.display());
                if i % 8 == 7 {
                    for j in i - 7..=i {
                        print!(" {}", format!("{: >2}", j).black());
                    }
                    println!();
                }
            }
            println!();
        }
    }
    fn play_random_game(
        &mut self,
        move_limit: u32,
    ) -> Result<GameState, String> {
        use rand::{seq::SliceRandom, thread_rng};
        for _ in 0..move_limit {
            let moves = self.gen_legal_moves()?;

            let mut rng = thread_rng();
            let mov = moves.choose(&mut rng);
            self.play_optional_move(mov).unwrap();
            if self.state.game_state != GameState::Playing { break }
        }
        Ok(self.state.game_state)
    }

    fn dbg_gen_moves(&mut self) -> Result<(), String> {
        let moves = self.gen_legal_moves()?;
        self.dbg_moves(&moves, vec![], true)?;
        Ok(())
    }

    fn squares(&self) -> [Square; 64] {
        self.squares
    }

    fn state(&self) -> BoardState {
        self.state
    }
}
