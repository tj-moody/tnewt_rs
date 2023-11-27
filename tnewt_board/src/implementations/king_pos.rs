use color_eyre::eyre::Result;
use colored::Colorize;

use crate::board::{self, KingIndices};
use board::Playable;

use crate::{castling, color, coordinate, mov, piece};

use color::Color;
use coordinate::Coordinate;
use mov::Move;
use piece::{Piece, Kind};

use crate::magic_numbers::{
    DIRECTION_OFFSETS,
    KNIGHT_THREAT_INDICES,
    SQUARES_TO_EDGE,
    get_pawn_threat_offsets,
    get_threat_pieces,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Board {
    pub squares: [Option<Piece>; 64],
    pub state: board::State,
    pub state_history: Vec<board::State>,
    pub history: Vec<[Option<Piece>; 64]>,
    pub algorithm: board::Algorithm,
    pub store_history: bool,
}

const STORE_HISTORY: bool = false;

impl Board {
    fn slider_gen_moves(
        &self,
        moves: &mut Vec<Move>,
        start_index: usize,
        piece: Piece,
    ) {
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
            None => return Err(board::Error::MoveEmptySquare),
        };

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
            return Ok(());
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
        Ok(())
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

    fn gen_pseudo_legal_moves(&self) -> Result<Vec<Move>, board::Error> {
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
                    Kind::Pawn => self.pawn_gen_moves(&mut moves, index)?,
                };
            }
        }
        self.gen_castling_moves(&mut moves);

        Ok(moves)
    }

    /// Only checks if the color whose turn it is has their piece at `index` being attacked
    fn is_attacked(&mut self, indices: &[usize], king_color: Color) -> Result<bool, board::Error> {
        for &start_index in indices {
            for &knight_index in KNIGHT_THREAT_INDICES[start_index] {
                if let Some(piece) = self.squares[knight_index] {
                    if piece.kind == Kind::Knight && piece.color != king_color {
                        return Ok(true);
                    }
                }
            }

            let pawn_threat_offsets = get_pawn_threat_offsets(king_color);
            for (direction_index, offset) in DIRECTION_OFFSETS.iter().enumerate() {
                let threat_pieces = get_threat_pieces(direction_index)?;
                for n in 0..SQUARES_TO_EDGE[start_index][direction_index] {
                    let target_index: usize = (start_index as i32 + (offset * (n + 1))) as usize;

                    let target_square = self.squares[target_index];

                    match target_square {
                        Some(piece) => {
                            if piece.color == king_color {
                                break;
                            }
                            if threat_pieces.contains(&piece.kind) {
                                return Ok(true);
                            }
                            if n == 0 {
                                if piece.kind == Kind::Pawn
                                    && pawn_threat_offsets.contains(offset)
                                {
                                    return Ok(true);
                                }
                                if piece.kind == Kind::King {
                                    return Ok(true);
                                }
                            }
                            break;
                        }
                        None => continue,
                    }
                }
            }
        }
        if true {
            return Ok(false);
        }

        Ok(false)
    }

    fn gen_king_indices(squares: &[Option<Piece>; 64]) -> Result<KingIndices, board::Error> {
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
            return Err(board::Error::NoKing);
        }
        Ok(KingIndices { white, black })
    }
}

impl Playable for Board {
    /// This function does not actually mutate self, as it calls `make_move`
    /// and `unmake_move` sequentially, without mutating anywhere else.
    fn gen_legal_moves(&mut self) -> Result<Vec<Move>, board::Error> {
        let mut moves = self.gen_pseudo_legal_moves()?;
        moves.retain(|mov| {
            let is_castling = mov.is_castling(&self.squares).unwrap();

            match self.algorithm {
                board::Algorithm::Clone => {
                    let mut board = self.clone();
                    board.make_move(mov).unwrap();

                    let king_index = board.state.king_indices.get(board.state.turn.opposite());
                    let check_indices = if is_castling {
                        castling::get_squares(mov).unwrap().check_indices.to_vec()
                    } else {
                        vec![king_index]
                    };

                    !board
                        .is_attacked(&check_indices, board.state.turn.opposite())
                        .unwrap()
                }
                board::Algorithm::Unmove => {
                    self.make_move(mov).unwrap();

                    let king_index = self.state.king_indices.get(self.state.turn.opposite());
                    let check_indices = if is_castling {
                        castling::get_squares(mov).unwrap().check_indices.to_vec()
                    } else {
                        vec![king_index]
                    };

                    let is_attacked = self
                        .is_attacked(&check_indices, self.state.turn.opposite())
                        .unwrap();
                    self.unmake_move().unwrap();
                    !is_attacked
                }
            }
        });
        Ok(moves)
    }

    fn from_chars(chars: &[char; 64]) -> Result<Self, board::Error> {
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
                _ => return Err(board::Error::InvalidPieceChar(*piece_char)),
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
                king_indices: Board::gen_king_indices(&squares)?,
            },
            state_history: vec![],
            history: vec![],
            algorithm: board::Algorithm::Clone,
            store_history: STORE_HISTORY,
        })
    }

    fn from_fen(fen: &str) -> Result<Self, board::Error> {
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
                    _ => return Err(board::Error::InvalidPieceChar(c)),
                }
            }
        }

        let array: [char; 64] = match squares.try_into() {
            Ok(some) => some,
            Err(_) => return Err(board::Error::InvalidFenBoardLength(fen.to_string())),
        };

        let mut board: Board = Board::from_chars(&array)?;
        board.state.turn = Color::from(turn)?;
        board.state.castling_state = castling::State::from(castling_rights);
        board.state.ep_index = Coordinate::from(ep_square)?.map(Coordinate::into_index);
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

    fn change_turn(&mut self) {
        self.state.turn = self.state.turn.opposite();
    }

    fn make_move(&mut self, mov: &Move) -> Result<(), board::Error> {
        let Some(moving_piece) = self.squares[mov.start_index] else {
            return Err(board::Error::MoveEmptySquare);
        };
        let captured_square = self.squares[mov.target_index];

        if self.algorithm == board::Algorithm::Unmove {
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

        if self.algorithm == board::Algorithm::Unmove {
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
                is_castling = mov.is_castling(&self.squares)?;
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
            self.state.game_state = board::GameState::Draw;
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
            let castling_squares = castling::get_squares(mov)?;
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

        Ok(())
    }

    fn unmake_move(&mut self) -> Result<(), board::Error> {
        let Some(last_captured_square) = self.state.last_captured_square else {
            return Err(board::Error::UndoFromFirstMove);
        };
        let Some(last_move) = self.state.last_move else {
            return Err(board::Error::UndoFromFirstMove);
        };
        let last_ep_taken_index = self.state.last_ep_taken_index;

        self.state = match self.state_history.pop() {
            Some(state) => state,
            None => return Err(board::Error::UndoFromFirstMove),
        };

        // Color of the player who made the move being undone
        let color = self.state.turn;

        if STORE_HISTORY {
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
        if last_move.is_castling(&self.squares)? {
            let castling_squares = match castling::get_squares(&last_move) {
                Ok(v) => v,
                Err(e) => {
                    self.display();
                    return Err(e);
                }
            };
            self.squares[castling_squares.rook_start_index] = Some(Piece {
                kind: Kind::Rook,
                color,
            });
            self.squares[castling_squares.rook_target_index] = None;
        }

        Ok(())
    }

    fn set_algorithm(&mut self, algorithm: board::Algorithm) {
        self.algorithm = algorithm;
    }

    fn set_castling_state(&mut self, rights: &str) {
        self.state.castling_state = castling::State::from(rights);
    }

    fn set_state(&mut self, state: board::State) {
        self.state = state;
    }

    fn display_history(&self) {
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
            println!("{:?}", self.state.turn);
        }
    }

    fn play_legal_move(&mut self, mov: Option<&Move>) -> Result<(), board::Error> {
        if let Some(mov) = mov { self.make_move(mov)? } else {
            let king_index = self.state.king_indices.get(self.state.turn);
            if self.is_attacked(&[king_index], self.state.turn)? {
                self.state.game_state = board::GameState::Victory(self.state.turn.opposite());
            } else {
                self.state.game_state = board::GameState::Draw;
            }
        };
        Ok(())
    }

    fn squares(&self) -> [Option<Piece>; 64] {
        self.squares
    }

    fn state(&self) -> board::State {
        self.state
    }

    fn algorithm(&self) -> board::Algorithm {
        self.algorithm
    }

    fn _history(&self) -> Vec<[Option<Piece>; 64]> {
        self.history.clone()
    }

    fn depth_num_positions(&mut self, depth: i32) -> Result<u32, board::Error> {
        if depth <= 0 {
            return Ok(1);
        }
        let moves = self.gen_legal_moves()?;
        let mut num_positions: u32 = 0;

        match self.algorithm() {
            board::Algorithm::Clone => {
                for mov in &moves {
                    let mut board = self.clone();
                    board.make_move(mov)?;
                    num_positions += board.depth_num_positions(depth - 1)?;
                }
            }
            board::Algorithm::Unmove => {
                for mov in &moves {
                    self.make_move(mov)?;
                    num_positions += self.depth_num_positions(depth - 1)?;
                    self.unmake_move()?;
                }
            }
        }

        Ok(num_positions)
    }
}
