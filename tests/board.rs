
#[cfg(test)]
mod tests {
    use tnewt_board::*;

    #[test]
    fn default_fen_functions() {
        let board = Board::new();
        let board2 = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(board, board2);
    }
    #[test]
    fn get_inbetween_moves() {
        let castling_moves = CastlingRights::from("kq").gen_moves(&Color::White).unwrap();
        let inbetween_squares = [
            vec![61, 62],
            vec![57, 58, 59],
        ];
        for i in 0..2 {
            if let Move::CastlingMove(cm) = castling_moves[i] {
                assert_eq!(cm.get_inbetween_squares(), inbetween_squares[i])
            } else { panic!("`CastlingRights.gen_moves()` returned `BasicMove`") }
        }
    }
}
