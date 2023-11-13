use color_eyre::eyre::Result;

use tnewt_board::*;

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

fn main() -> Result<(), String> {
    let implementation_string = std::env::args().nth(1);
    let mut implementation: Option<Implementation> = None;
    if implementation_string == Some("Unmove".to_string()) {
        implementation = Some(Implementation::Unmove);
    } else if implementation_string == Some("Clone".to_string()) {
        implementation = Some(Implementation::Clone);
    }

    // let mut board = Board::from(&[
    //     ' ',' ',' ',' ',' ',' ',' ',' ', //  0  1  2  3  4  5  6  7
    //     ' ',' ',' ',' ',' ',' ',' ',' ', //  8  9 10 11 12 13 14 15
    //     ' ',' ',' ',' ',' ',' ',' ',' ', // 16 17 18 19 20 21 22 23
    //     ' ',' ',' ',' ',' ',' ',' ',' ', // 24 25 26 27 28 29 30 31
    //     ' ',' ',' ',' ',' ',' ',' ',' ', // 32 33 34 35 36 37 38 39
    //     ' ',' ',' ',' ',' ',' ',' ',' ', // 40 41 42 43 44 45 46 47
    //     ' ',' ',' ',' ',' ',' ',' ',' ', // 48 49 50 51 52 53 54 55
    //     ' ',' ',' ',' ',' ',' ',' ',' ', // 56 57 58 59 60 61 62 63
    // ])?;

    let mut board = Board::new();
    match implementation {
        Some(implementation) => board.implementation = implementation,
        None => (),
    }
    dbg!(board.implementation);


    // for i in 0..1000 {
    //     let mut board = Board::new();
    //     board.implementation = Implementation::Unmove;
    //     println!("Game {i}: {:?}", board.play_random_game(500)?);
    // }

    // board.display();
    // // board.gen_legal_moves()?;
    // board.dbg_gen_moves()?;

    for i in 1..6 {
        dbg!(board.dbg_depth_num_positions(i)?);
    }

    Ok(())
}
