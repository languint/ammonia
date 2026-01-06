use crate::chess::{nrof::NrOf, piece};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece(u8);
impl Piece {
    pub const PAWN: Piece = Piece(0);
    pub const KNIGHT: Piece = Piece(1);
    pub const BISHOP: Piece = Piece(2);
    pub const ROOK: Piece = Piece(3);
    pub const QUEEN: Piece = Piece(4);
    pub const KING: Piece = Piece(5);

    pub const ALL: [Piece; NrOf::PIECES] = [
        Piece::PAWN,
        Piece::KNIGHT,
        Piece::BISHOP,
        Piece::ROOK,
        Piece::QUEEN,
        Piece::KING,
    ];
}
