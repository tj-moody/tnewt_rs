mod board;
use board::{Board, Move, Piece};

fn main() {
    let board = Board::from(&[
        'r','n','b','q','k','b','n','r',
        'p','p','p','p','p','p','p','p',
        ' ',' ',' ',' ',' ',' ',' ',' ',
        ' ',' ',' ',' ',' ',' ',' ',' ',
        ' ',' ',' ',' ',' ',' ',' ',' ',
        ' ',' ',' ',' ',' ',' ',' ',' ',
        'P','P','P','P','P','P','P','P',
        'R','N','B','Q','K','B','N','R',
    ]);
    let moves = board.generate_moves();
    Move::debug_moves(
        &moves,
        vec![Piece::Pawn],
        false,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_fen_functions() {
        let board = Board::new();
        let board2 = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(board, board2);
    }
}
