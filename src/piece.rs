use crate::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece(pub u8);
impl Piece {
    pub const PAWN: Piece = Piece(0b001);
    pub const KNIGHT: Piece = Piece(0b010);
    pub const BISHOP: Piece = Piece(0b011);
    pub const ROOK: Piece = Piece(0b100);
    pub const QUEEN: Piece = Piece(0b101);
    pub const KING: Piece = Piece(0b110);

    pub const NONE: Piece = Piece(0b000);

    pub const COLOR_SHIFT: usize = 3;
}

impl Piece {
    /// Construct a new [`Piece`] from color and type bits.
    pub fn new(color: Color, piece_type: Piece) -> Piece {
        Piece((color.0 << Piece::COLOR_SHIFT) | piece_type.0)
    }
}

impl Piece {
    /// Retrieve the color bits of a [`Piece`]
    pub fn get_color(&self) -> Option<Color> {
        if self == &Piece::NONE {
            return None;
        }
        Some(Color(self.0 >> Piece::COLOR_SHIFT))
    }

    /// Retrieve the type bits of a [`Piece`]
    pub fn get_type(&self) -> Piece {
        Piece(self.0 & 0b00111)
    }
}

impl Piece {
    pub fn index(self) -> usize {
        match self.get_type() {
            Piece::PAWN => 0,
            Piece::KNIGHT => 1,
            Piece::BISHOP => 2,
            Piece::ROOK => 3,
            Piece::QUEEN => 4,
            Piece::KING => 5,
            _ => unreachable!("Piece::index called on NONE or invalid piece"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Color, piece::Piece};

    #[test]
    fn construct_piece() {
        let piece = Piece::new(Color::WHITE, Piece::PAWN);

        assert_eq!(piece.0, 0b01001);

        let piece = Piece::new(Color::BLACK, Piece::PAWN);

        assert_eq!(piece.0, 0b10001);
    }

    #[test]
    fn accessors() {
        let piece = Piece::new(Color::WHITE, Piece::PAWN);

        assert_eq!(piece.get_color(), Some(Color::WHITE));
        assert_eq!(piece.get_type(), Piece::PAWN);
    }
}
