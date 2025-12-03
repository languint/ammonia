pub struct NrOf;
impl NrOf {
    pub const SQUARES: usize = 64;
    pub const FILES: usize = 8;
    pub const RANKS: usize = 8;
}

/// A file on the chess board, [A-H]->[0-7]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct File(pub u8);
impl File {
    pub const A: File = File(0);
    pub const B: File = File(1);
    pub const C: File = File(2);
    pub const D: File = File(3);
    pub const E: File = File(4);
    pub const F: File = File(5);
    pub const G: File = File(6);
    pub const H: File = File(7);

    pub const ALL_FILES: [File; NrOf::FILES] = [
        File::A,
        File::B,
        File::C,
        File::D,
        File::E,
        File::F,
        File::G,
        File::H,
    ];
}

/// A rank on the chess board, [1-8]->[0-7]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rank(pub u8);
impl Rank {
    pub const ONE: Rank = Rank(0);
    pub const TWO: Rank = Rank(1);
    pub const THREE: Rank = Rank(2);
    pub const FOUR: Rank = Rank(3);
    pub const FIVE: Rank = Rank(4);
    pub const SIX: Rank = Rank(5);
    pub const SEVEN: Rank = Rank(6);
    pub const EIGHT: Rank = Rank(7);

    pub const ALL_RANKS: [Rank; NrOf::RANKS] = [
        Rank::ONE,
        Rank::TWO,
        Rank::THREE,
        Rank::FOUR,
        Rank::FIVE,
        Rank::SIX,
        Rank::SEVEN,
        Rank::EIGHT,
    ];
}

/// A square on the chess board [0-63]->[a1-h8]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square(pub u8);
impl Square {
    pub const ALL_SQUARES: [Square; NrOf::SQUARES] = {
        let mut squares: [Square; NrOf::SQUARES] = [Square(0); NrOf::SQUARES];
        let mut i = 0;

        while i < NrOf::SQUARES {
            squares[i] = Square(i as u8);
            i += 1;
        }

        squares
    };

    /// Construct a square from a [`Rank`] and [`File`]
    pub fn from_rank_file(rank: Rank, file: File) -> Square {
        Square((rank.0 * 8) + file.0)
    }

    pub fn get_rank(&self) -> Rank {
        Rank(self.0 / 8)
    }

    pub fn get_file(&self) -> File {
        File(self.0 % 8)
    }
}

#[cfg(test)]
mod tests {
    use crate::board::defs::{File, NrOf, Rank, Square};

    #[test]
    fn square_from_u8() {
        for i in 0..NrOf::SQUARES {
            let sq = Square(i as u8);

            assert_eq!(sq.get_file(), File((i % 8) as u8));
            assert_eq!(sq.get_rank(), Rank((i / 8) as u8));
        }
    }

    #[test]
    fn square_accessors() {
        let a1 = Square(0);
        let h8 = Square(63);

        assert_eq!(a1.get_file(), File::A);
        assert_eq!(a1.get_rank(), Rank::ONE);

        assert_eq!(h8.get_file(), File::H);
        assert_eq!(h8.get_rank(), Rank::EIGHT);
    }
}
