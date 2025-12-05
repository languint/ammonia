#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights(pub u8);
impl CastlingRights {
    pub const WK: CastlingRights = CastlingRights(0b0001);
    pub const WQ: CastlingRights = CastlingRights(0b0010);
    pub const BK: CastlingRights = CastlingRights(0b0100);
    pub const BQ: CastlingRights = CastlingRights(0b1000);
    pub const ALL: CastlingRights = CastlingRights(0b1111);
    pub const NONE: CastlingRights = CastlingRights(0b0000);
}

impl std::ops::BitOrAssign for CastlingRights {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl std::ops::BitAndAssign for CastlingRights {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl std::ops::BitAnd for CastlingRights {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for CastlingRights {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::Not for CastlingRights {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
