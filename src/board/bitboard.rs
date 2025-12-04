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

impl Default for Bitboard {
    fn default() -> Self {
        Bitboard::EMPTY
    }
}

impl Bitboard {
    #[inline]
    pub fn get_bit(self, square: Square) -> bool {
        (self.0 & (1u64 << square.0)) != 0
    }

    #[inline]
    pub fn set_bit(&mut self, square: Square, value: bool) {
        if value {
            self.0 |= 1u64 << square.0;
        } else {
            self.0 &= !(1u64 << square.0);
        }
    }
}

impl Bitboard {
    pub fn pop(self) -> u32 {
        self.0.count_ones()
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn lsb(self) -> Option<u8> {
        if self.0 == 0 {
            None
        } else {
            Some(self.0.trailing_zeros() as u8)
        }
    }

    pub fn pop_lsb(&mut self) -> Option<u8> {
        let lsb = self.lsb()?;
        self.0 &= self.0 - 1;
        Some(lsb)
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitXor for Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl std::ops::Not for Bitboard {
    type Output = Bitboard;
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
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
        bitboard.set_bit(Square(0), true);
        assert_eq!(bitboard.get_bit(Square(0)), true);
    }
}
