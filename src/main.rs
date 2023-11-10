use tnewt_board::Board;

fn main() {
    if match std::env::args().nth(1) {
        Some(arg) => arg == "Profile".to_string(),
        None => false,
    } {
    }
    #[allow(unused_mut)]
    let mut board = Board::from(&[
        'r','n','b','q','k','b','n','r', //  0  1  2  3  4  5  6  7
        'p','p','p','p','p','p','p','p', //  8  9 10 11 12 13 14 15
        ' ',' ',' ',' ',' ',' ',' ',' ', // 16 17 18 19 20 21 22 23
        ' ',' ',' ',' ',' ',' ',' ',' ', // 24 25 26 27 28 29 30 31
        ' ',' ',' ',' ',' ',' ',' ',' ', // 32 33 34 35 36 37 38 39
        ' ',' ',' ',' ',' ',' ',' ',' ', // 40 41 42 43 44 45 46 47
        'P','P','P','P','P','P','P','P', // 48 49 50 51 52 53 54 55
        'R','N','B','Q','K','B','N','R', // 56 57 58 59 60 61 62 63
    ]).unwrap();
    // board.play_move(&[52, 36]).unwrap();
    // board.play_move(&[12, 28]).unwrap();

    let moves = board.gen_legal_moves().unwrap();
    tnewt_board::Move::debug_moves(&moves, vec![tnewt_board::PieceKind::Pawn], true);
    // board.display();
}
