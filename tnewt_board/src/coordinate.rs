use crate::board::Error;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Coordinate {
    A1, A2, A3, A4, A5, A6, A7, A8,
    B1, B2, B3, B4, B5, B6, B7, B8,
    C1, C2, C3, C4, C5, C6, C7, C8,
    D1, D2, D3, D4, D5, D6, D7, D8,
    E1, E2, E3, E4, E5, E6, E7, E8,
    F1, F2, F3, F4, F5, F6, F7, F8,
    G1, G2, G3, G4, G5, G6, G7, G8,
    H1, H2, H3, H4, H5, H6, H7, H8,
}
impl Coordinate {
    #[must_use]
    pub fn from(coordinate: &str) -> Result<Option<Self>, Error> {
        match coordinate {// {{{
            "a1" => Ok(Some(Coordinate::A1)),
            "a2" => Ok(Some(Coordinate::A2)),
            "a3" => Ok(Some(Coordinate::A3)),
            "a4" => Ok(Some(Coordinate::A4)),
            "a5" => Ok(Some(Coordinate::A5)),
            "a6" => Ok(Some(Coordinate::A6)),
            "a7" => Ok(Some(Coordinate::A7)),
            "a8" => Ok(Some(Coordinate::A8)),
            "b1" => Ok(Some(Coordinate::B1)),
            "b2" => Ok(Some(Coordinate::B2)),
            "b3" => Ok(Some(Coordinate::B3)),
            "b4" => Ok(Some(Coordinate::B4)),
            "b5" => Ok(Some(Coordinate::B5)),
            "b6" => Ok(Some(Coordinate::B6)),
            "b7" => Ok(Some(Coordinate::B7)),
            "b8" => Ok(Some(Coordinate::B8)),
            "c1" => Ok(Some(Coordinate::C1)),
            "c2" => Ok(Some(Coordinate::C2)),
            "c3" => Ok(Some(Coordinate::C3)),
            "c4" => Ok(Some(Coordinate::C4)),
            "c5" => Ok(Some(Coordinate::C5)),
            "c6" => Ok(Some(Coordinate::C6)),
            "c7" => Ok(Some(Coordinate::C7)),
            "c8" => Ok(Some(Coordinate::C8)),
            "d1" => Ok(Some(Coordinate::D1)),
            "d2" => Ok(Some(Coordinate::D2)),
            "d3" => Ok(Some(Coordinate::D3)),
            "d4" => Ok(Some(Coordinate::D4)),
            "d5" => Ok(Some(Coordinate::D5)),
            "d6" => Ok(Some(Coordinate::D6)),
            "d7" => Ok(Some(Coordinate::D7)),
            "d8" => Ok(Some(Coordinate::D8)),
            "e1" => Ok(Some(Coordinate::E1)),
            "e2" => Ok(Some(Coordinate::E2)),
            "e3" => Ok(Some(Coordinate::E3)),
            "e4" => Ok(Some(Coordinate::E4)),
            "e5" => Ok(Some(Coordinate::E5)),
            "e6" => Ok(Some(Coordinate::E6)),
            "e7" => Ok(Some(Coordinate::E7)),
            "e8" => Ok(Some(Coordinate::E8)),
            "f1" => Ok(Some(Coordinate::F1)),
            "f2" => Ok(Some(Coordinate::F2)),
            "f3" => Ok(Some(Coordinate::F3)),
            "f4" => Ok(Some(Coordinate::F4)),
            "f5" => Ok(Some(Coordinate::F5)),
            "f6" => Ok(Some(Coordinate::F6)),
            "f7" => Ok(Some(Coordinate::F7)),
            "f8" => Ok(Some(Coordinate::F8)),
            "g1" => Ok(Some(Coordinate::G1)),
            "g2" => Ok(Some(Coordinate::G2)),
            "g3" => Ok(Some(Coordinate::G3)),
            "g4" => Ok(Some(Coordinate::G4)),
            "g5" => Ok(Some(Coordinate::G5)),
            "g6" => Ok(Some(Coordinate::G6)),
            "g7" => Ok(Some(Coordinate::G7)),
            "g8" => Ok(Some(Coordinate::G8)),
            "h1" => Ok(Some(Coordinate::H1)),
            "h2" => Ok(Some(Coordinate::H2)),
            "h3" => Ok(Some(Coordinate::H3)),
            "h4" => Ok(Some(Coordinate::H4)),
            "h5" => Ok(Some(Coordinate::H5)),
            "h6" => Ok(Some(Coordinate::H6)),
            "h7" => Ok(Some(Coordinate::H7)),
            "h8" => Ok(Some(Coordinate::H8)),
            "-"  => Ok(None),
            _    => Err(Error::InvalidCoordinate(coordinate.to_string())),
        }// }}}
    }
    #[must_use]
    pub fn into_index(self) -> usize {
        match self {// {{{
            Coordinate::A8 => 0,
            Coordinate::B8 => 1,
            Coordinate::C8 => 2,
            Coordinate::D8 => 3,
            Coordinate::E8 => 4,
            Coordinate::F8 => 5,
            Coordinate::G8 => 6,
            Coordinate::H8 => 7,
            Coordinate::A7 => 8,
            Coordinate::B7 => 9,
            Coordinate::C7 => 10,
            Coordinate::D7 => 11,
            Coordinate::E7 => 12,
            Coordinate::F7 => 13,
            Coordinate::G7 => 14,
            Coordinate::H7 => 15,
            Coordinate::A6 => 16,
            Coordinate::B6 => 17,
            Coordinate::C6 => 18,
            Coordinate::D6 => 19,
            Coordinate::E6 => 20,
            Coordinate::F6 => 21,
            Coordinate::G6 => 22,
            Coordinate::H6 => 23,
            Coordinate::A5 => 24,
            Coordinate::B5 => 25,
            Coordinate::C5 => 26,
            Coordinate::D5 => 27,
            Coordinate::E5 => 28,
            Coordinate::F5 => 29,
            Coordinate::G5 => 30,
            Coordinate::H5 => 31,
            Coordinate::A4 => 32,
            Coordinate::B4 => 33,
            Coordinate::C4 => 34,
            Coordinate::D4 => 35,
            Coordinate::E4 => 36,
            Coordinate::F4 => 37,
            Coordinate::G4 => 38,
            Coordinate::H4 => 39,
            Coordinate::A3 => 40,
            Coordinate::B3 => 41,
            Coordinate::C3 => 42,
            Coordinate::D3 => 43,
            Coordinate::E3 => 44,
            Coordinate::F3 => 45,
            Coordinate::G3 => 46,
            Coordinate::H3 => 47,
            Coordinate::A2 => 48,
            Coordinate::B2 => 49,
            Coordinate::C2 => 50,
            Coordinate::D2 => 51,
            Coordinate::E2 => 52,
            Coordinate::F2 => 53,
            Coordinate::G2 => 54,
            Coordinate::H2 => 55,
            Coordinate::A1 => 56,
            Coordinate::B1 => 57,
            Coordinate::C1 => 58,
            Coordinate::D1 => 59,
            Coordinate::E1 => 60,
            Coordinate::F1 => 61,
            Coordinate::G1 => 62,
            Coordinate::H1 => 63,
        }// }}}
    }
    #[must_use]
    pub fn from_index(index: usize) -> Self {
        match index {// {{{
            0 => Coordinate::A8,
            1 => Coordinate::B8,
            2 => Coordinate::C8,
            3 => Coordinate::D8,
            4 => Coordinate::E8,
            5 => Coordinate::F8,
            6 => Coordinate::G8,
            7 => Coordinate::H8,
            8 => Coordinate::A7,
            9 => Coordinate::B7,
            10 => Coordinate::C7,
            11 => Coordinate::D7,
            12 => Coordinate::E7,
            13 => Coordinate::F7,
            14 => Coordinate::G7,
            15 => Coordinate::H7,
            16 => Coordinate::A6,
            17 => Coordinate::B6,
            18 => Coordinate::C6,
            19 => Coordinate::D6,
            20 => Coordinate::E6,
            21 => Coordinate::F6,
            22 => Coordinate::G6,
            23 => Coordinate::H6,
            24 => Coordinate::A5,
            25 => Coordinate::B5,
            26 => Coordinate::C5,
            27 => Coordinate::D5,
            28 => Coordinate::E5,
            29 => Coordinate::F5,
            30 => Coordinate::G5,
            31 => Coordinate::H5,
            32 => Coordinate::A4,
            33 => Coordinate::B4,
            34 => Coordinate::C4,
            35 => Coordinate::D4,
            36 => Coordinate::E4,
            37 => Coordinate::F4,
            38 => Coordinate::G4,
            39 => Coordinate::H4,
            40 => Coordinate::A3,
            41 => Coordinate::B3,
            42 => Coordinate::C3,
            43 => Coordinate::D3,
            44 => Coordinate::E3,
            45 => Coordinate::F3,
            46 => Coordinate::G3,
            47 => Coordinate::H3,
            48 => Coordinate::A2,
            49 => Coordinate::B2,
            50 => Coordinate::C2,
            51 => Coordinate::D2,
            52 => Coordinate::E2,
            53 => Coordinate::F2,
            54 => Coordinate::G2,
            55 => Coordinate::H2,
            56 => Coordinate::A1,
            57 => Coordinate::B1,
            58 => Coordinate::C1,
            59 => Coordinate::D1,
            60 => Coordinate::E1,
            61 => Coordinate::F1,
            62 => Coordinate::G1,
            63 => Coordinate::H1,
            _  => panic!("Invalid index"),
        }// }}}
    }
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {// {{{
            Coordinate::A1 => "a1",
            Coordinate::A2 => "a2",
            Coordinate::A3 => "a3",
            Coordinate::A4 => "a4",
            Coordinate::A5 => "a5",
            Coordinate::A6 => "a6",
            Coordinate::A7 => "a7",
            Coordinate::A8 => "a8",
            Coordinate::B1 => "b1",
            Coordinate::B2 => "b2",
            Coordinate::B3 => "b3",
            Coordinate::B4 => "b4",
            Coordinate::B5 => "b5",
            Coordinate::B6 => "b6",
            Coordinate::B7 => "b7",
            Coordinate::B8 => "b8",
            Coordinate::C1 => "c1",
            Coordinate::C2 => "c2",
            Coordinate::C3 => "c3",
            Coordinate::C4 => "c4",
            Coordinate::C5 => "c5",
            Coordinate::C6 => "c6",
            Coordinate::C7 => "c7",
            Coordinate::C8 => "c8",
            Coordinate::D1 => "d1",
            Coordinate::D2 => "d2",
            Coordinate::D3 => "d3",
            Coordinate::D4 => "d4",
            Coordinate::D5 => "d5",
            Coordinate::D6 => "d6",
            Coordinate::D7 => "d7",
            Coordinate::D8 => "d8",
            Coordinate::E1 => "e1",
            Coordinate::E2 => "e2",
            Coordinate::E3 => "e3",
            Coordinate::E4 => "e4",
            Coordinate::E5 => "e5",
            Coordinate::E6 => "e6",
            Coordinate::E7 => "e7",
            Coordinate::E8 => "e8",
            Coordinate::F1 => "f1",
            Coordinate::F2 => "f2",
            Coordinate::F3 => "f3",
            Coordinate::F4 => "f4",
            Coordinate::F5 => "f5",
            Coordinate::F6 => "f6",
            Coordinate::F7 => "f7",
            Coordinate::F8 => "f8",
            Coordinate::G1 => "g1",
            Coordinate::G2 => "g2",
            Coordinate::G3 => "g3",
            Coordinate::G4 => "g4",
            Coordinate::G5 => "g5",
            Coordinate::G6 => "g6",
            Coordinate::G7 => "g7",
            Coordinate::G8 => "g8",
            Coordinate::H1 => "h1",
            Coordinate::H2 => "h2",
            Coordinate::H3 => "h3",
            Coordinate::H4 => "h4",
            Coordinate::H5 => "h5",
            Coordinate::H6 => "h6",
            Coordinate::H7 => "h7",
            Coordinate::H8 => "h8",
        })// }}}
    }
}
