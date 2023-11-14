#[cfg(test)]

mod tests {
    use tnewt_board::*;

    use board::{PlayableBoard, Algorithm};
    use color::*;
    use square::*;

    const TEST_FENS: [(&str, [u32; 3]); 6] = [
        ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",                 [20, 400 , 8902 ]),
        ("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",     [48, 2039, 97862]),
        ("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",                                [14, 191 , 2812 ]),
        ("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",         [6 , 264 , 9467 ]),
        ("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",                [44, 1486, 62379]),
        ("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", [46, 2079, 89890])
    ];


    #[test]
    fn default_fen_functions() -> Result<(), String> {
        let board = board::new();
        let board2 = board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")?;
        assert_eq!(board.squares(), board2.squares());
        assert_eq!(board.state(), board2.state());
        Ok(())
    }

    #[test]
    fn is_same_color() {
        let white_square1 = Square::Some(Piece {
            kind: PieceKind::Queen,
            color: Color::White
        });
        let white_square2 = Square::Some(Piece {
            kind: PieceKind::Pawn,
            color: Color::White
        });
        let black_square1 = Square::Some(Piece {
            kind: PieceKind::King,
            color: Color::Black
        });
        let black_square2 = Square::Some(Piece {
            kind: PieceKind::Rook,
            color: Color::Black
        });
        let empty_square = Square::Empty;
        assert!(!white_square1.is_same_color(black_square1));
        assert!(!white_square1.is_same_color(empty_square));
        assert!(white_square1.is_same_color(white_square2));

        assert!(!black_square1.is_same_color(white_square1));
        assert!(!black_square1.is_same_color(empty_square));
        assert!(black_square1.is_same_color(black_square2));
    }

    #[test]
    fn clone_depth_3_num_positions() -> Result<(), String> {
        for (i, (fen, num_positions)) in TEST_FENS.into_iter().enumerate() {
            let mut board = board::from_fen(fen)?;
            board.dbg_set_algorithm(Algorithm::Clone);
            println!("Test Position {i}");
            board.display();
            for j in 0..3 {
                assert_eq!(board.dbg_depth_num_positions(j as i32 + 1)?, num_positions[j]);
            }
        }
        Ok(())
    }

    #[test]
    fn unmove_depth_3_num_positions() -> Result<(), String> {
        for (i, (fen, num_positions)) in TEST_FENS.into_iter().enumerate() {
            let mut board = board::from_fen(fen)?;
            board.dbg_set_algorithm(Algorithm::Unmove);
            println!("Test Position {i}");
            board.display();
            for j in 0..3 {
                assert_eq!(board.dbg_depth_num_positions(j as i32 + 1)?, num_positions[j]);
            }
        }
        Ok(())
    }

    #[test]
    fn play_random_moves() -> Result<(), String> {
        let mut board = board::new();
        const DEPTH: u32 = 10_000;
        let _game_state = board.play_random_game(DEPTH)?;
        Ok(())
    }
}
