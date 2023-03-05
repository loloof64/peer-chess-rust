use pleco::{File, Rank};

pub struct Utils {}

impl Utils {
    pub fn coord_file_to_pleco_file(input: i32) -> File {
        let constrained_input = input.max(0).min(7);
        match constrained_input {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => File::A,
        }
    }

    pub fn coord_rank_to_pleco_rank(input: i32) -> Rank {
        let constrained_input = input.max(0).min(7);
        match constrained_input {
            0 => Rank::R1,
            1 => Rank::R2,
            2 => Rank::R3,
            3 => Rank::R4,
            4 => Rank::R5,
            5 => Rank::R6,
            6 => Rank::R7,
            7 => Rank::R8,
            _ => Rank::R1,
        }
    }
}