/// A chess piece, a basic wrapper around `u8`
#[derive(Debug)]
pub struct Piece(pub u8);
impl Piece {
    pub const PAWN: u8 = 0b001;
    pub const KNIGHT: u8 = 0b010;
    pub const BISHOP: u8 = 0b011;
    pub const ROOK: u8 = 0b100;
    pub const QUEEN: u8 = 0b101;
    pub const KING: u8 = 0b110;

    pub const NONE: u8 = 0b000;

    pub const COLOR_SHIFT: usize = 3;
    pub const COLOR_WHITE: u8 = 0b01;
    pub const COLOR_BLACK: u8 = 0b10;
}

impl Piece {
    /// Retrieve the color bits of a [`Piece`]
    pub fn get_color(&self) -> u8 {
        self.0 >> Piece::COLOR_SHIFT
    }

    /// Retrieve the type bits of a [`Piece`]
    pub fn get_type(&self) -> u8 {
        self.0 & 0b00111
    }
}

impl Piece {
    /// Construct a new [`Piece`] from color and type bits.
    pub fn new(color: u8, piece_type: u8) -> Piece {
        Piece((color << Piece::COLOR_SHIFT) | piece_type)
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

        assert_eq!(piece.get_color(), Color::WHITE);
        assert_eq!(piece.get_type(), Piece::PAWN);
    }
}
