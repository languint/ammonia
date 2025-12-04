use crate::{
    board::{
        bitboard::Bitboard,
        defs::{NrOf, Square},
    },
    color::Color,
    game::game_move::{Move, MoveFlag},
    piece::Piece,
};

pub mod bitboard;
pub mod defs;

pub struct Board {
    pub bb_pieces: [Bitboard; NrOf::PIECES],
    pub bb_colors: [Bitboard; NrOf::COLORS],
    pub pieces: [Piece; NrOf::SQUARES],
    pub color_to_move: Color,
}
impl Board {
    pub const EMPTY: Board = Board {
        bb_pieces: [Bitboard::EMPTY; NrOf::PIECES],
        bb_colors: [Bitboard::EMPTY; NrOf::COLORS],
        pieces: [Piece::NONE; NrOf::SQUARES],
        color_to_move: Color::WHITE,
    };
}

impl Board {
    #[inline]
    pub fn piece_bb(&self, piece_index: usize) -> Bitboard {
        self.bb_pieces[piece_index]
    }

    pub fn color_bb(&self, color_index: usize) -> Bitboard {
        self.bb_colors[color_index]
    }

    pub fn all_occupied(&self) -> Bitboard {
        let mut bb = Bitboard::EMPTY;
        for &b in &self.bb_colors {
            bb |= b;
        }
        bb
    }

    pub fn is_square_occupied(&self, square: Square) -> bool {
        (self.all_occupied() & Bitboard::square_mask(square)) != Bitboard::EMPTY
    }
}

#[derive(Debug, PartialEq)]
pub enum MakeMoveError {
    NoCaptureVictim,
    UnhandledMoveFlag(MoveFlag),
    InvalidPieceColor,
    MissingFlag(MoveFlag),
}

impl Board {
    pub fn make_move(&mut self, mv: Move) -> Result<(), MakeMoveError> {
        let src = mv.get_source();
        let dest = mv.get_dest();
        let flag = mv.get_flag();

        let source_index = src.0 as usize;
        let dest_index = dest.0 as usize;

        let moving_piece = self.pieces[source_index];
        if moving_piece == Piece::NONE {
            return Err(MakeMoveError::InvalidPieceColor);
        }
        let color = moving_piece
            .get_color()
            .ok_or(MakeMoveError::InvalidPieceColor)?;
        let color_index = color.index();
        let piece_index = moving_piece.index();

        let src_mask = Bitboard::square_mask(src);
        let dest_mask = Bitboard::square_mask(dest);

        self.bb_pieces[piece_index] &= !src_mask;
        self.bb_colors[color_index] &= !src_mask;
        self.pieces[source_index] = Piece::NONE;

        match flag {
            MoveFlag::CAPTURE => {
                let captured_piece = self.pieces[dest_index];
                if captured_piece == Piece::NONE {
                    return Err(MakeMoveError::NoCaptureVictim);
                }
                let captured_piece_index = captured_piece.index();
                let captured_color_index = captured_piece
                    .get_color()
                    .ok_or(MakeMoveError::InvalidPieceColor)?
                    .index();

                self.bb_pieces[captured_piece_index] &= !dest_mask;
                self.bb_colors[captured_color_index] &= !dest_mask;
                self.pieces[dest_index] = Piece::NONE;
            }

            MoveFlag::EN_PASSANT => {
                let capture_square = if color == Color::WHITE {
                    Square(dest.0 - 8)
                } else {
                    Square(dest.0 + 8)
                };

                let capture_index = capture_square.0 as usize;
                let captured_piece = self.pieces[capture_index];
                let captured_type = captured_piece.index();
                let captured_color_index = captured_piece
                    .get_color()
                    .ok_or(MakeMoveError::InvalidPieceColor)?
                    .index();

                let mask = Bitboard::square_mask(capture_square);
                self.bb_pieces[captured_type] &= !mask;
                self.bb_colors[captured_color_index] &= !mask;
                self.pieces[capture_index] = Piece::NONE;
            }

            MoveFlag::CASTLING => {
                let (rook_src, rook_dest) = match dest {
                    Square::G1 => (Square::H1, Square::F1),
                    Square::C1 => (Square::A1, Square::D1),
                    Square::G8 => (Square::H8, Square::F8),
                    Square::C8 => (Square::A8, Square::D8),
                    _ => unreachable!("Invalid castling move"),
                };

                let rook_piece = self.pieces[rook_src.0 as usize];
                let rook_piece_index = rook_piece.index();
                let rook_color_index = color_index;

                let src_mask = Bitboard::square_mask(rook_src);
                let dest_mask = Bitboard::square_mask(rook_dest);

                self.bb_pieces[rook_piece_index] &= !src_mask;
                self.bb_colors[rook_color_index] &= !src_mask;
                self.bb_pieces[rook_piece_index] |= dest_mask;
                self.bb_colors[rook_color_index] |= dest_mask;

                self.pieces[rook_dest.0 as usize] = rook_piece;
                self.pieces[rook_src.0 as usize] = Piece::NONE;
            }
            MoveFlag::PROMOTION_BISHOP
            | MoveFlag::PROMOTION_KNIGHT
            | MoveFlag::PROMOTION_ROOK
            | MoveFlag::PROMOTION_QUEEN => {
                let pawn_piece = self.pieces[src.0 as usize];
                let pawn_piece_index = pawn_piece.index();
                let pawn_color_index = color_index;

                let src_mask = Bitboard::square_mask(src);
                let dest_mask = Bitboard::square_mask(dest);

                self.bb_pieces[pawn_piece_index] &= !src_mask;
                self.bb_colors[pawn_color_index] &= !src_mask;
                self.bb_pieces[pawn_piece_index] |= dest_mask;
                self.bb_colors[pawn_color_index] |= dest_mask;

                let promoted_piece_type = match flag {
                    MoveFlag::PROMOTION_KNIGHT => Piece::KNIGHT,
                    MoveFlag::PROMOTION_BISHOP => Piece::BISHOP,
                    MoveFlag::PROMOTION_ROOK => Piece::ROOK,
                    MoveFlag::PROMOTION_QUEEN => Piece::QUEEN,
                    _ => unreachable!(),
                };

                let promoted_piece = Piece::new(color, promoted_piece_type);
                let promoted_piece_index = promoted_piece_type.index();

                self.bb_pieces[promoted_piece_index] |= dest_mask;
                self.bb_colors[color_index] |= dest_mask;
                self.pieces[dest_index] = promoted_piece;
            }
            MoveFlag::NONE => {}
            _ => return Err(MakeMoveError::UnhandledMoveFlag(flag)),
        }

        self.bb_pieces[piece_index] |= dest_mask;
        self.bb_colors[color_index] |= dest_mask;
        self.pieces[dest_index] = moving_piece;

        self.color_to_move = !self.color_to_move;

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::defs::Square;

    #[test]
    fn test_empty_board() {
        let board = Board::EMPTY;

        for i in 0..NrOf::PIECES {
            assert_eq!(board.piece_bb(i), Bitboard::EMPTY);
        }

        for i in 0..NrOf::COLORS {
            assert_eq!(board.color_bb(i), Bitboard::EMPTY);
        }

        assert_eq!(board.all_occupied(), Bitboard::EMPTY);

        for sq in 0..64 {
            let square = Square(sq);
            assert!(!board.is_square_occupied(square));
        }
    }

    #[test]
    fn test_single_piece() {
        let mut board = Board::EMPTY;

        let sq = Square(0);
        let piece_index = 0;
        let color_index = 0;
        board.bb_pieces[piece_index] = Bitboard::square_mask(sq);
        board.bb_colors[color_index] = Bitboard::square_mask(sq);

        assert_eq!(board.piece_bb(piece_index), Bitboard::square_mask(sq));
        assert_eq!(board.color_bb(color_index), Bitboard::square_mask(sq));

        assert_eq!(board.all_occupied(), Bitboard::square_mask(sq));

        assert!(board.is_square_occupied(sq));
        assert!(!board.is_square_occupied(Square(1)));
    }

    #[test]
    fn test_multiple_pieces() {
        let mut board = Board::EMPTY;

        let sq1 = Square(0);
        let sq2 = Square(63);

        board.bb_pieces[0] = Bitboard::square_mask(sq1);
        board.bb_pieces[1] = Bitboard::square_mask(sq2);

        board.bb_colors[0] = Bitboard::square_mask(sq1);
        board.bb_colors[1] = Bitboard::square_mask(sq2);

        let occupied = board.all_occupied();

        assert!(board.is_square_occupied(sq1));
        assert!(board.is_square_occupied(sq2));

        let expected = Bitboard::square_mask(sq1) | Bitboard::square_mask(sq2);
        assert_eq!(occupied, expected);
    }

    #[test]
    fn test_simple_move() {
        let mut board = Board::EMPTY;

        let sq_from = Square(0);
        let sq_to = Square(1);
        board.pieces[sq_from.0 as usize] = Piece::new(Color::WHITE, Piece::PAWN);
        let piece_index = Piece::PAWN.index();
        board.bb_pieces[piece_index] = Bitboard::square_mask(sq_from);
        board.bb_colors[Color::WHITE.index()] = Bitboard::square_mask(sq_from);

        let mv = Move::new(MoveFlag::NONE, sq_from, sq_to);
        board.make_move(mv).expect("Expected move to succeed");

        assert_eq!(board.pieces[sq_from.0 as usize], Piece::NONE);
        assert_eq!(
            board.pieces[sq_to.0 as usize],
            Piece::new(Color::WHITE, Piece::PAWN)
        );

        assert_eq!(board.bb_pieces[piece_index], Bitboard::square_mask(sq_to));
        assert_eq!(
            board.bb_colors[Color::WHITE.index()],
            Bitboard::square_mask(sq_to)
        );

        assert_eq!(board.color_to_move, Color::BLACK);
    }

    #[test]
    fn test_capture_move() {
        let mut board = Board::EMPTY;

        let sq_from = Square(0);
        let sq_to = Square(1);
        board.pieces[sq_from.0 as usize] = Piece::new(Color::WHITE, Piece::PAWN);
        board.bb_pieces[Piece::PAWN.index()] |= Bitboard::square_mask(sq_from);
        board.bb_colors[Color::WHITE.index()] |= Bitboard::square_mask(sq_from);

        board.pieces[sq_to.0 as usize] = Piece::new(Color::BLACK, Piece::PAWN);
        board.bb_pieces[Piece::PAWN.index()] |= Bitboard::square_mask(sq_to);
        board.bb_colors[Color::BLACK.index()] |= Bitboard::square_mask(sq_to);

        let mv = Move::new(MoveFlag::CAPTURE, sq_from, sq_to);
        board.make_move(mv).expect("Expected move to succeed");

        assert_eq!(board.pieces[sq_from.0 as usize], Piece::NONE);
        assert_eq!(
            board.pieces[sq_to.0 as usize],
            Piece::new(Color::WHITE, Piece::PAWN)
        );

        assert_eq!(
            board.bb_colors[Color::BLACK.index()] & Bitboard::square_mask(sq_to),
            Bitboard::EMPTY
        );

        assert_eq!(board.color_to_move, Color::BLACK);
    }

    #[test]
    fn test_multiple_moves() {
        let mut board = Board::EMPTY;

        let sq1 = Square(0);
        let sq2 = Square(1);
        board.pieces[sq1.0 as usize] = Piece::new(Color::WHITE, Piece::PAWN);
        board.bb_pieces[Piece::PAWN.index()] |= Bitboard::square_mask(sq1);
        board.bb_colors[Color::WHITE.index()] |= Bitboard::square_mask(sq1);

        board.pieces[sq2.0 as usize] = Piece::new(Color::WHITE, Piece::KNIGHT);
        board.bb_pieces[Piece::KNIGHT.index()] |= Bitboard::square_mask(sq2);
        board.bb_colors[Color::WHITE.index()] |= Bitboard::square_mask(sq2);

        let mv = Move::new(MoveFlag::NONE, sq1, Square(2));
        board.make_move(mv).unwrap();

        assert_eq!(board.pieces[sq1.0 as usize], Piece::NONE);
        assert_eq!(
            board.pieces[Square(2).0 as usize],
            Piece::new(Color::WHITE, Piece::PAWN)
        );

        assert_eq!(
            board.pieces[sq2.0 as usize],
            Piece::new(Color::WHITE, Piece::KNIGHT)
        );
    }

    #[test]
    fn test_en_passant_white() {
        let mut board = Board::EMPTY;

        let wp_sq = Square(28);
        board.pieces[wp_sq.0 as usize] = Piece::new(Color::WHITE, Piece::PAWN);
        board.bb_pieces[Piece::PAWN.index()] |= Bitboard::square_mask(wp_sq);
        board.bb_colors[Color::WHITE.index()] |= Bitboard::square_mask(wp_sq);

        let bp_sq = Square(27);
        board.pieces[bp_sq.0 as usize] = Piece::new(Color::BLACK, Piece::PAWN);
        board.bb_pieces[Piece::PAWN.index()] |= Bitboard::square_mask(bp_sq);
        board.bb_colors[Color::BLACK.index()] |= Bitboard::square_mask(bp_sq);

        let ep_move = Move::new(MoveFlag::EN_PASSANT, wp_sq, Square(35));
        board.make_move(ep_move).expect("En passant should succeed");

        assert_eq!(board.pieces[bp_sq.0 as usize], Piece::NONE);
        assert_eq!(
            board.bb_colors[Color::BLACK.index()] & Bitboard::square_mask(bp_sq),
            Bitboard::EMPTY
        );

        assert_eq!(board.pieces[35], Piece::new(Color::WHITE, Piece::PAWN));
    }

    #[test]
    fn test_en_passant_black() {
        let mut board = Board::EMPTY;

        let bp_sq = Square::D5;
        board.pieces[bp_sq.0 as usize] = Piece::new(Color::BLACK, Piece::PAWN);
        board.bb_pieces[Piece::PAWN.index()] |= Bitboard::square_mask(bp_sq);
        board.bb_colors[Color::BLACK.index()] |= Bitboard::square_mask(bp_sq);

        let wp_sq = Square::E5;
        board.pieces[wp_sq.0 as usize] = Piece::new(Color::WHITE, Piece::PAWN);
        board.bb_pieces[Piece::PAWN.index()] |= Bitboard::square_mask(wp_sq);
        board.bb_colors[Color::WHITE.index()] |= Bitboard::square_mask(wp_sq);

        board.color_to_move = Color::BLACK;

        let ep_move = Move::new(MoveFlag::EN_PASSANT, bp_sq, Square::E4);
        board.make_move(ep_move).expect("En passant should succeed");

        assert_eq!(board.pieces[wp_sq.0 as usize], Piece::NONE);
        assert_eq!(
            board.bb_colors[Color::WHITE.index()] & Bitboard::square_mask(wp_sq),
            Bitboard::EMPTY
        );

        assert_eq!(
            board.pieces[Square::E4.0 as usize],
            Piece::new(Color::BLACK, Piece::PAWN)
        );
    }

    #[test]
    fn test_castling_kingside_white() {
        let mut board = Board::EMPTY;

        let king_sq = Square::E1;
        let rook_sq = Square::H1;

        board.pieces[king_sq.0 as usize] = Piece::new(Color::WHITE, Piece::KING);
        board.pieces[rook_sq.0 as usize] = Piece::new(Color::WHITE, Piece::ROOK);

        board.bb_pieces[Piece::KING.index()] |= Bitboard::square_mask(king_sq);
        board.bb_pieces[Piece::ROOK.index()] |= Bitboard::square_mask(rook_sq);

        board.bb_colors[Color::WHITE.index()] |=
            Bitboard::square_mask(king_sq) | Bitboard::square_mask(rook_sq);

        let castle_move = Move::new(MoveFlag::CASTLING, king_sq, Square::G1);
        board
            .make_move(castle_move)
            .expect("Castling should succeed");

        assert_eq!(
            board.pieces[Square::G1.0 as usize],
            Piece::new(Color::WHITE, Piece::KING)
        );
        assert_eq!(
            board.pieces[Square::F1.0 as usize],
            Piece::new(Color::WHITE, Piece::ROOK)
        );

        assert_eq!(board.pieces[king_sq.0 as usize], Piece::NONE);
        assert_eq!(board.pieces[rook_sq.0 as usize], Piece::NONE);
    }

    #[test]
    fn test_castling_queenside_black() {
        let mut board = Board::EMPTY;

        let king_sq = Square::E8;
        let rook_sq = Square::A8;
        board.pieces[king_sq.0 as usize] = Piece::new(Color::BLACK, Piece::KING);
        board.pieces[rook_sq.0 as usize] = Piece::new(Color::BLACK, Piece::ROOK);
        board.bb_pieces[Piece::KING.index()] |= Bitboard::square_mask(king_sq);
        board.bb_pieces[Piece::ROOK.index()] |= Bitboard::square_mask(rook_sq);
        board.bb_colors[Color::BLACK.index()] |=
            Bitboard::square_mask(king_sq) | Bitboard::square_mask(rook_sq);

        board.color_to_move = Color::BLACK;

        let castle_move = Move::new(MoveFlag::CASTLING, king_sq, Square::C8);
        board
            .make_move(castle_move)
            .expect("Castling should succeed");

        assert_eq!(
            board.pieces[Square::C8.0 as usize],
            Piece::new(Color::BLACK, Piece::KING)
        );
        assert_eq!(
            board.pieces[Square::D8.0 as usize],
            Piece::new(Color::BLACK, Piece::ROOK)
        );

        assert_eq!(board.pieces[king_sq.0 as usize], Piece::NONE);
        assert_eq!(board.pieces[rook_sq.0 as usize], Piece::NONE);
    }
}
