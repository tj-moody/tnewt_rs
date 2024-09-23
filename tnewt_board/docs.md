# Public interface for a `Board`
    /// Generates a Vec of all legal moves in the current position.
    pub fn gen_legal_moves(&mut self) -> Vec<Move>

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
    pub fn from_chars(chars: &[char; 64]) -> Result<Self, Error>

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
    pub fn from_fen(fen: &str) -> Result<Self, Error>

    /// Change the board's current turn.
    fn change_turn(&mut self)
        self.state.turn = self.state.turn.opposite();
    }

    /// Play [`mov`] on the board, and update the state and history accordingly.
    /// [`mov`] must be legal.
    ///
    /// # Errors
    ///
    /// This function will return an error if [`mov`] attempts to move from an empty square
    /// or is illegal.
    pub fn make_move(&mut self, mov: &Move)

    /// Undo the board's most recent move.
    ///
    /// # Errors
    ///
    /// This function will return an error if the board is on its first move.
    pub fn unmake_move(&mut self)

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
    pub fn set_algorithm(&mut self, algorithm: Algorithm)

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
    pub fn play_legal_move(&mut self, mov: Option<&Move>)

    /// Get the number of possible positions after a certain [`depth`.]
    /// A [`depth`] of 0 gives 1, and a depth of 1 gives the current number of legal moves.
    ///
    /// Only requires a mutable reference when the [`unmove`] algorithm is being used.
    pub fn depth_num_positions(&mut self, depth: i32) -> u32

    /// Creates a new [`Board`] in the starting position.
    pub fn new() -> Self

    /// Returns the current position and state in FEN notation.
    /// See: [Forsyth-Edwards_Notation](https://en.wikipedia.org/wiki/Forsyth-Edwards_Notation)
    pub fn to_fen(&self) -> &str

    /// Prints the current position in a human-readable format.
    pub fn display(&self)

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
    pub fn display_moves(&self, moves: &[Move], shown_pieces: Vec<Kind>)

    /// Returns the index of the [`color`'s] king.
    ///
    /// # Errors
    ///
    /// This function will return an error if player [`color`] does not have a king.
    pub fn king_index(&self, color: Color) -> Result<usize, Error>

    /// Generates the board's current legal moves and displays them in a
    /// human-readable format.
    ///
    /// Only requires a mutable reference when the [`unmove`] algorithm is being used.
    ///
    /// # Errors
    ///
    /// This function will return an error if move generation fails.
    pub fn gen_and_display_moves(&mut self)

    /// Move piece from [`start_index`] to [`target_index`.]
    ///
    /// # Errors
    ///
    /// This function will return an error if [`start_index`] or [`target_index`] are not within
    /// 0..64, if [`start_index`] is an empty square, or if the move is not legal.
    pub fn dbg_play_move(

    /// Get the current number of legal moves.
    ///
    /// Only requires a mutable reference when the [`unmove`] algorithm is being used.
    ///
    /// # Errors
    ///
    /// This function will return an error if move generation fails.
    pub fn num_legal_moves(&mut self) -> Result<usize, Error>

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
    pub fn perft(&mut self, depth: i32) -> Result<u32, Error>

    /// Display the position history of the board in a human-readable format.
    /// History is only stored if the [`DEBUG_HISTORY`] global is set to true.
    pub fn display_history(&self)

    /// Play a random game up to [`move_limit`] moves.
    /// Will leave the board in the last position of the game.
    pub fn play_random_game(&mut self, move_limit: u32) -> Result<GameState, Error>
}
