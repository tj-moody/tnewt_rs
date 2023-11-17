#![allow(unused_mut, unused_variables, unused_imports)]
use color_eyre::eyre::Result;

use tnewt_board::board;
use board::Playable;

#[allow(dead_code)]
static STARTING_POSITION: &[char; 64] = &[
    'r','n','b','q','k','b','n','r', //  0  1  2  3  4  5  6  7
    'p','p','p','p','p','p','p','p', //  8  9 10 11 12 13 14 15
    ' ',' ',' ',' ',' ',' ',' ',' ', // 16 17 18 19 20 21 22 23
    ' ',' ',' ',' ',' ',' ',' ',' ', // 24 25 26 27 28 29 30 31
    ' ',' ',' ',' ',' ',' ',' ',' ', // 32 33 34 35 36 37 38 39
    ' ',' ',' ',' ',' ',' ',' ',' ', // 40 41 42 43 44 45 46 47
    'P','P','P','P','P','P','P','P', // 48 49 50 51 52 53 54 55
    'R','N','B','Q','K','B','N','R', // 56 57 58 59 60 61 62 63
];
#[allow(dead_code)]
static EMPTY_POSITION: &[char; 64] = &[
    ' ',' ',' ',' ',' ',' ',' ',' ', //  0  1  2  3  4  5  6  7
    ' ',' ',' ',' ',' ',' ',' ',' ', //  8  9 10 11 12 13 14 15
    ' ',' ',' ',' ',' ',' ',' ',' ', // 16 17 18 19 20 21 22 23
    ' ',' ',' ',' ',' ',' ',' ',' ', // 24 25 26 27 28 29 30 31
    ' ',' ',' ',' ',' ',' ',' ',' ', // 32 33 34 35 36 37 38 39
    ' ',' ',' ',' ',' ',' ',' ',' ', // 40 41 42 43 44 45 46 47
    ' ',' ',' ',' ',' ',' ',' ',' ', // 48 49 50 51 52 53 54 55
    ' ',' ',' ',' ',' ',' ',' ',' ', // 56 57 58 59 60 61 62 63
];

fn main() -> Result<(), board::Error> {

    // let mut board = board::from_chars(&[
    //     'k',' ',' ',' ','r',' ',' ',' ', //  0  1  2  3  4  5  6  7
    //     ' ',' ',' ',' ',' ',' ',' ',' ', //  8  9 10 11 12 13 14 15
    //     ' ',' ',' ',' ',' ',' ',' ',' ', // 16 17 18 19 20 21 22 23
    //     ' ',' ',' ',' ',' ',' ',' ',' ', // 24 25 26 27 28 29 30 31
    //     ' ',' ',' ',' ',' ',' ',' ',' ', // 32 33 34 35 36 37 38 39
    //     ' ',' ',' ',' ','B',' ',' ',' ', // 40 41 42 43 44 45 46 47
    //     ' ',' ',' ',' ',' ',' ',' ',' ', // 48 49 50 51 52 53 54 55
    //     ' ',' ',' ',' ','K',' ',' ',' ', // 56 57 58 59 60 61 62 63
    // ])?;


    // for i in 0..1000 {
    //     let mut board = Board::new();
    //     board.algorithm = Algorithm::Unmove;
    //     println!("Game {i}: {:?}", board.play_random_game(500)?);
    // }

    let mut board = board::from_fen(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    )?;
    board.dbg_set_algorithm(board::Algorithm::Clone);
    board.dbg_move_count()?;
    board.display();
    println!();
    board.dbg_set_algorithm(board::Algorithm::Unmove);
    board.dbg_move_count()?;
    board.display();
    // board.dbg_gen_moves()?;

    // for i in 1..15 {
    //     println!("{}", board.dbg_depth_num_positions(i)?);
    // }

    Ok(())
}
