use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub const ALL: [Piece; 6] = [
        Piece::Pawn,
        Piece::Knight,
        Piece::Bishop,
        Piece::Rook,
        Piece::Queen,
        Piece::King,
    ];
}

impl TryFrom<u8> for Piece {
    type Error = String;
    fn try_from(value: u8) -> Result<Piece, String> {
        if value > 5 {
            return Err(format!("{value} out of range for Piece::try_from(u8)"));
        }

        Ok(unsafe { std::mem::transmute::<u8, Piece>(value) })
    }
}

pub struct PieceParseError(String);

impl std::fmt::Display for PieceParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse piece: {}", self.0)
    }
}

impl FromStr for Piece {
    type Err = PieceParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "p" => Ok(Piece::Pawn),
            "n" => Ok(Piece::Knight),
            "b" => Ok(Piece::Bishop),
            "r" => Ok(Piece::Rook),
            "q" => Ok(Piece::Queen),
            "k" => Ok(Piece::King),
            _ => Err(PieceParseError(format!("{s} is not a piece"))),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::chess::piece::Piece;

    #[test]
    fn test_from_u8() {
        for (index, piece) in Piece::ALL.iter().enumerate() {
            assert_eq!(
                &Piece::try_from(index as u8).expect("This should succeed"),
                piece
            );
        }
    }
}
