#![allow(unused_mut, unused_variables, unused_imports)]
use color_eyre::eyre::Result;

use tnewt_board::board;
use tnewt_board::board::Board;
use tnewt_board::implementations;
use tnewt_board::mov::Move;

fn main() -> Result<(), board::Error> {
    // let mut board = initialize::from_chars!(&[
    //     ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', //  0  1  2  3  4  5  6  7
    //     ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', //  8  9 10 11 12 13 14 15
    //     ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 16 17 18 19 20 21 22 23
    //     ' ', ' ', ' ', 'k', ' ', ' ', ' ', ' ', // 24 25 26 27 28 29 30 31
    //     ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 32 33 34 35 36 37 38 39
    //     ' ', ' ', ' ', 'K', ' ', ' ', ' ', ' ', // 40 41 42 43 44 45 46 47
    //     ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 48 49 50 51 52 53 54 55
    //     ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // 56 57 58 59 60 61 62 63
    // ])?;

    let mut board = Board::new();
    board.perft(5)?;

    Ok(())
}
