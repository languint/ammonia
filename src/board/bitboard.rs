use crate::board::defs::{File, Rank, Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);
impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard(u64::MAX);

    #[inline]
    pub const fn square_mask(square: Square) -> Bitboard {
        Bitboard(1u64 << square.0)
    }

    #[inline]
    pub const fn file_mask(file: File) -> Bitboard {
        const FILE_A: u64 = 0x101010101010101;

        Bitboard(FILE_A << file.0)
    }

    #[inline]
    pub const fn rank_mask(rank: Rank) -> Bitboard {
        const RANK_ONE: u64 = 0xFF;

        Bitboard(RANK_ONE << (rank.0 * 8))
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Bitboard::EMPTY
    }
}

impl From<u64> for Bitboard {
    fn from(v: u64) -> Self {
        Bitboard(v)
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

/// A direction of a bitboard
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Direction(pub i8);
impl Direction {
    pub const NORTH: Direction = Direction(8);
    pub const SOUTH: Direction = Direction(-8);
    pub const WEST: Direction = Direction(-1);
    pub const EAST: Direction = Direction(1);

    pub const NORTH_EAST: Direction = Direction::NORTH.add_dir(Direction::EAST);
    pub const NORTH_WEST: Direction = Direction::NORTH.add_dir(Direction::WEST);
    pub const SOUTH_EAST: Direction = Direction::SOUTH.add_dir(Direction::EAST);
    pub const SOUTH_WEST: Direction = Direction::SOUTH.add_dir(Direction::WEST);

    pub const ALL_DIRECTIONS: [Direction; 8] = [
        Direction::NORTH_WEST,
        Direction::NORTH,
        Direction::NORTH_EAST,
        Direction::EAST,
        Direction::SOUTH_EAST,
        Direction::SOUTH,
        Direction::SOUTH_WEST,
        Direction::WEST,
    ];
}

impl std::ops::Shr<Direction> for Bitboard {
    type Output = Bitboard;
    fn shr(self, rhs: Direction) -> Self::Output {
        Bitboard(
            self.0
                >> match rhs {
                    Direction::NORTH_WEST => Direction::NORTH_WEST.0,
                    Direction::NORTH => Direction::NORTH.0,
                    Direction::NORTH_EAST => Direction::NORTH_EAST.0,
                    Direction::EAST => Direction::EAST.0,
                    Direction::SOUTH_EAST => -Direction::SOUTH_EAST.0,
                    Direction::SOUTH => -Direction::SOUTH.0,
                    Direction::SOUTH_WEST => -Direction::SOUTH_WEST.0,
                    Direction::WEST => -Direction::WEST.0,
                    _ => unreachable!(),
                },
        )
    }
}

impl std::ops::Shl<Direction> for Bitboard {
    type Output = Bitboard;
    fn shl(self, rhs: Direction) -> Self::Output {
        Bitboard(
            self.0
                << match rhs {
                    Direction::NORTH_WEST => Direction::NORTH_WEST.0,
                    Direction::NORTH => Direction::NORTH.0,
                    Direction::NORTH_EAST => Direction::NORTH_EAST.0,
                    Direction::EAST => Direction::EAST.0,
                    Direction::SOUTH_EAST => -Direction::SOUTH_EAST.0,
                    Direction::SOUTH => -Direction::SOUTH.0,
                    Direction::SOUTH_WEST => -Direction::SOUTH_WEST.0,
                    Direction::WEST => -Direction::WEST.0,
                    _ => unreachable!(),
                },
        )
    }
}

impl Direction {
    pub const fn add_dir(&self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Add for Direction {
    type Output = Direction;
    fn add(self, rhs: Self) -> Self::Output {
        self.add_dir(rhs)
    }
}

impl Bitboard {
    pub const NOT_A_FILE: u64 = !0x0101010101010101;
    pub const NOT_H_FILE: u64 = !0x8080808080808080;
    pub const NOT_AB_FILE: u64 = !0x0303030303030303;
    pub const NOT_GH_FILE: u64 = !0xC0C0C0C0C0C0C0C0;

    #[inline(always)]
    pub fn north_one(self) -> Self {
        Bitboard(self.0 << 8)
    }

    #[inline(always)]
    pub fn south_one(self) -> Self {
        Bitboard(self.0 >> 8)
    }

    #[inline(always)]
    pub fn south_east_one(self) -> Self {
        Bitboard((self.0 >> 7) & Self::NOT_A_FILE)
    }

    #[inline(always)]
    pub fn south_west_one(self) -> Self {
        Bitboard((self.0 >> 9) & Self::NOT_H_FILE)
    }

    #[inline(always)]
    pub fn north_east_one(self) -> Self {
        Bitboard((self.0 << 9) & Self::NOT_A_FILE)
    }

    #[inline(always)]
    pub fn north_west_one(self) -> Self {
        Bitboard((self.0 << 7) & Self::NOT_H_FILE)
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
