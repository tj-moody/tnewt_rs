#![allow(unused_mut, unused_variables, unused_imports)]
use color_eyre::eyre::Result;

use board::Playable;
use tnewt_board::{board, from_chars};
use tnewt_board::board::Algorithm;
use tnewt_board::implementations;
use tnewt_board::mov::Move;

#[allow(dead_code)]
static STARTING_POSITION: &[char; 64] = &[
    'r', 'n', 'b', 'q', 'k', 'b', 'n', 'r', //  0  1  2  3  4  5  6  7
    'p', 'p', 'p', 'p', 'p', 'p', 'p', 'p', //  8  9 10 11 12 13 14 15
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 16 17 18 19 20 21 22 23
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 24 25 26 27 28 29 30 31
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 32 33 34 35 36 37 38 39
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 40 41 42 43 44 45 46 47
    'P', 'P', 'P', 'P', 'P', 'P', 'P', 'P', // 48 49 50 51 52 53 54 55
    'R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R', // 56 57 58 59 60 61 62 63
];
#[allow(dead_code)]
static EMPTY_POSITION: &[char; 64] = &[
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', //  0  1  2  3  4  5  6  7
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', //  8  9 10 11 12 13 14 15
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 16 17 18 19 20 21 22 23
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 24 25 26 27 28 29 30 31
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 32 33 34 35 36 37 38 39
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 40 41 42 43 44 45 46 47
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 48 49 50 51 52 53 54 55
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 56 57 58 59 60 61 62 63
];

fn main() -> Result<(), board::Error> {
    let mut board = tnewt_board::from_fen!(threat_squares, "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8")?;
    // after a2a3
    let mut board = tnewt_board::from_fen!(threat_squares, "rnbq1k1r/pp1Pbppp/2p5/8/2B5/P7/1PP1NnPP/RNBQK2R b KQ - 1 8")?;
    // after d8a5
    let mut board = tnewt_board::from_fen!(threat_squares, "rnb2k1r/pp1Pbppp/2p5/q7/2B5/P7/1PP1NnPP/RNBQK2R w KQ - 1 8")?;

    board.set_algorithm(Algorithm::Clone);

    board.display();
    println!();
    board.perft(1)?;

    Ok(())
}
