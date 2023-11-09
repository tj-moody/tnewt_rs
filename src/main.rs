pub mod board;
use board::Board;

fn main() {
    if match std::env::args().nth(1) {
        Some(arg) => arg == "Profile".to_string(),
        None => false,
    } {
    }
    let board = Board::new();
    let moves = board.generate_moves().unwrap();
    board::Move::debug_moves(&moves, vec![board::PieceKind::Pawn], false,);
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
