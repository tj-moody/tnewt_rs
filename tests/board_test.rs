#[cfg(test)]

mod tests {
    use tnewt_board::*;

    use board::{Playable, Algorithm};
    use color::*;
    use piece::*;

    const TEST_FENS: [(&str, [u32; 3]); 6] = [
        ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",                 [20, 400 , 8902 ]),
        ("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",     [48, 2039, 97862]),
        ("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",                                [14, 191 , 2812 ]),
        ("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",         [6 , 264 , 9467 ]),
        ("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",                [44, 1486, 62379]),
        ("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", [46, 2079, 89890])
    ];

    fn new() -> impl Playable {
        board::new::<implementations::pre_filter::Board>()
    }

    fn from_fen(fen: &str) -> Result<impl Playable, board::Error> {
        board::from_fen::<implementations::pre_filter::Board>(fen)
    }


    #[test]
    fn default_fen_functions() -> Result<(), board::Error> {
        let board = new();
        let board2 = from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")?;
        assert_eq!(board.squares(), board2.squares());
        assert_eq!(board.state(), board2.state());
        Ok(())
    }

    #[test]
    fn is_same_color() {
        let white_square1 = Some(Piece {
            kind: PieceKind::Queen,
            color: Color::White
        });
        let white_square2 = Some(Piece {
            kind: PieceKind::Pawn,
            color: Color::White
        });
        let black_square1 = Some(Piece {
            kind: PieceKind::King,
            color: Color::Black
        });
        let black_square2 = Some(Piece {
            kind: PieceKind::Rook,
            color: Color::Black
        });
        let empty_square = None;
        assert!(!Piece::is_same_color(white_square1, black_square1));
        assert!(!Piece::is_same_color(white_square1, black_square1));
        assert!(!Piece::is_same_color(white_square1, empty_square));
        assert!(Piece::is_same_color(white_square1, white_square2));

        assert!(!Piece::is_same_color(black_square1, white_square1));
        assert!(!Piece::is_same_color(black_square1, empty_square));
        assert!(Piece::is_same_color(black_square1, black_square2));
    }

    #[test]
    fn clone_depth_3_num_positions() -> Result<(), board::Error> {
        for (i, (fen, num_positions)) in TEST_FENS.into_iter().enumerate() {
            let mut board = from_fen(fen)?;
            board.set_algorithm(Algorithm::Clone);
            println!("Test Position {i}");
            board.gen_legal_moves()?;
            board.display();
            for (j, &num_position) in num_positions.iter().enumerate() {
                assert_eq!(board.depth_num_positions(j as i32 + 1)?, num_position);
            }
        }
        Ok(())
    }

    #[test]
    fn unmove_depth_3_num_positions() -> Result<(), board::Error> {
        for (i, (fen, num_positions)) in TEST_FENS.into_iter().enumerate() {
            let mut board = from_fen(fen)?;
            board.set_algorithm(Algorithm::Unmove);
            println!("Test Position {i}");
            board.display();
            for (j, &num_position) in num_positions.iter().enumerate() {
                println!("{j}");
                assert_eq!(board.depth_num_positions(j as i32 + 1)?, num_position);
            }
        }
        Ok(())
    }

    #[test]
    fn clone_play_random_games() -> Result<(), board::Error> {
        let mut board = new();
        board.set_algorithm(Algorithm::Clone);
        const DEPTH: u32 = 10_000;
        for _ in 0..10_000 {
            let game_state = board.play_random_game(DEPTH);
            assert!(game_state.is_ok())
        }
        Ok(())
    }
    #[test]
    fn unmove_play_random_games() -> Result<(), board::Error> {
        let mut board = new();
        board.set_algorithm(Algorithm::Unmove);
        const DEPTH: u32 = 10_000;
        for _ in 0..10_000 {
            let game_state = board.play_random_game(DEPTH);
            assert!(game_state.is_ok())
        }
        Ok(())
    }
}
