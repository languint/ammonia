use crate::board::defs::{File, Rank, Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);
impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard(u64::MAX);

    #[inline]
    pub fn square_mask(square: Square) -> Bitboard {
        Bitboard(1u64 << square.0)
    }

    #[inline]
    pub fn file_mask(file: File) -> Bitboard {
        const FILE_A: u64 = 0x101010101010101;

        Bitboard(FILE_A << file.0)
    }

    #[inline]
    pub fn rank_mask(rank: Rank) -> Bitboard {
        const RANK_ONE: u64 = 0xFF;

        Bitboard(RANK_ONE << (rank.0 * 8))
    }
}

impl Bitboard {
    #[inline]
    pub fn get_bit(self, square: Square) -> u64 {
        self.0 & (1u64 << square.0)
    }

    #[inline]
    pub fn set_bit(&mut self, square: Square, value: u64) {
        self.0 |= value << square.0;
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{
        bitboard::Bitboard,
        defs::{File, Rank, Square},
    };

    #[test]
    fn square_masks() {
        for sq in Square::ALL_SQUARES {
            assert_eq!(Bitboard::square_mask(sq), Bitboard(1u64 << sq.0));
        }
    }

    #[test]
    fn file_masks() {
        assert_eq!(Bitboard::file_mask(File::A), Bitboard(0x101010101010101));
        assert_eq!(Bitboard::file_mask(File::H), Bitboard(0x8080808080808080));
    }

    #[test]
    fn rank_masks() {
        assert_eq!(Bitboard::rank_mask(Rank::ONE), Bitboard(0xFF));
        assert_eq!(
            Bitboard::rank_mask(Rank::EIGHT),
            Bitboard(0xFF00000000000000)
        );
    }

    #[test]
    fn accessors() {
        let mut bitboard = Bitboard::EMPTY;
        bitboard.set_bit(Square(0), 1);
        assert_eq!(bitboard.get_bit(Square(0)), 1);
    }
}
