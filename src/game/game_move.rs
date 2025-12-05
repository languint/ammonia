use crate::board::defs::Square;

/// A struct holding a `u16` where the 4 most significant bytes represent the [`MoveFlag`],
/// and the 12 least significant bits represent the source and destination [`Square`]
///
/// FFFFSSSSSSDDDDDD
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move(pub u16);
impl Move {
    pub const FLAG_SHIFT: usize = 12;
    pub const SOURCE_SHIFT: usize = 6;

    #[inline]
    pub fn new(flag: MoveFlag, source: Square, destination: Square) -> Move {
        let f = u16::from(flag.0) << Move::FLAG_SHIFT;
        let s = u16::from(source.0) << Move::SOURCE_SHIFT;
        let d = u16::from(destination.0);

        Move(f | s | d)
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let middle_chars = match self.get_flag() {
            MoveFlag::CAPTURE => "x",
            _ => "",
        };
        let end_chars = match self.get_flag() {
            MoveFlag::PROMOTION_KNIGHT => "=N",
            MoveFlag::PROMOTION_BISHOP => "=B",
            MoveFlag::PROMOTION_ROOK => "=R",
            MoveFlag::PROMOTION_QUEEN => "=Q",
            _ => "",
        };

        write!(
            f,
            "{}{}{}{}",
            self.get_source().get_algebraic(),
            middle_chars,
            self.get_dest().get_algebraic(),
            end_chars
        )
    }
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let middle_chars = match self.get_flag() {
            MoveFlag::CAPTURE => "x",
            _ => "",
        };
        let end_chars = match self.get_flag() {
            MoveFlag::PROMOTION_KNIGHT => "=N",
            MoveFlag::PROMOTION_BISHOP => "=B",
            MoveFlag::PROMOTION_ROOK => "=R",
            MoveFlag::PROMOTION_QUEEN => "=Q",
            _ => "",
        };

        write!(
            f,
            "{}{}{}{}",
            self.get_source().get_algebraic(),
            middle_chars,
            self.get_dest().get_algebraic(),
            end_chars
        )
    }
}

impl Move {
    #[inline]
    pub fn get_flag(self) -> MoveFlag {
        MoveFlag((self.0 >> Move::FLAG_SHIFT) as u8)
    }

    #[inline]
    pub fn get_source(self) -> Square {
        Square(((self.0 >> Move::SOURCE_SHIFT) & 0b111111) as u8)
    }

    #[inline]
    pub fn get_dest(self) -> Square {
        Square(((self.0) & 0b111111) as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveFlag(pub u8);
impl MoveFlag {
    pub const NONE: MoveFlag = MoveFlag(0b0000);
    pub const CAPTURE: MoveFlag = MoveFlag(0b0001);
    pub const EN_PASSANT: MoveFlag = MoveFlag(0b0010);
    pub const CASTLING: MoveFlag = MoveFlag(0b0011);
    pub const PROMOTION_KNIGHT: MoveFlag = MoveFlag(0b0100);
    pub const PROMOTION_BISHOP: MoveFlag = MoveFlag(0b0101);
    pub const PROMOTION_ROOK: MoveFlag = MoveFlag(0b0110);
    pub const PROMOTION_QUEEN: MoveFlag = MoveFlag(0b0111);
}

#[cfg(test)]
mod tests {
    use crate::{
        board::defs::Square,
        game::game_move::{Move, MoveFlag},
    };

    #[test]
    fn move_construction() {
        let game_move = Move::new(MoveFlag::EN_PASSANT, Square::D5, Square::E6);
        assert_eq!(game_move.0, 0b0010100011101100);
    }

    #[test]
    fn accessors() {
        let game_move = Move::new(MoveFlag::EN_PASSANT, Square::D5, Square::E6);

        assert_eq!(game_move.get_flag(), MoveFlag::EN_PASSANT);
        assert_eq!(game_move.get_source(), Square::D5);
        assert_eq!(game_move.get_dest(), Square::E6);
    }
}
