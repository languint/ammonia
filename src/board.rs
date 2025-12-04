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
}

impl Board {
    pub fn make_move(&mut self, mv: Move) -> Result<(), MakeMoveError> {
        let src = mv.get_source();
        let dst = mv.get_dest();
        let flag = mv.get_flag();

        let src_idx = src.0 as usize;
        let dest_idx = dst.0 as usize;

        let moving_piece = self.pieces[src_idx];
        let color_index = moving_piece.get_color().index();
        let piece_index = usize::from(moving_piece.get_type().0);

        let src_mask = Bitboard::square_mask(src);
        let dest_mask = Bitboard::square_mask(dst);

        self.bb_pieces[piece_index] &= !src_mask;
        self.bb_colors[color_index] &= !src_mask;
        self.pieces[src_idx] = Piece::NONE;

        if flag == MoveFlag::CAPTURE {
            let captured_piece = self.pieces[dest_idx];
            if captured_piece == Piece::NONE {
                return Err(MakeMoveError::NoCaptureVictim);
            }
            let cap_piece_idx = captured_piece.get_type().0 as usize;
            let cap_color_idx = captured_piece.get_color().index();
            self.bb_pieces[cap_piece_idx] &= !dest_mask;
            self.bb_colors[cap_color_idx] &= !dest_mask;
        }

        self.bb_pieces[piece_index] |= dest_mask;
        self.bb_colors[color_index] |= dest_mask;
        self.pieces[dest_idx] = moving_piece;

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
        let piece_index = Piece::PAWN.0 as usize;
        board.bb_pieces[piece_index] = Bitboard::square_mask(sq_from);
        board.bb_colors[Color::WHITE.0 as usize] = Bitboard::square_mask(sq_from);

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
        board.bb_pieces[Piece::PAWN.0 as usize] |= Bitboard::square_mask(sq_from);
        board.bb_colors[Color::WHITE.index()] |= Bitboard::square_mask(sq_from);

        board.pieces[sq_to.0 as usize] = Piece::new(Color::BLACK, Piece::PAWN);
        board.bb_pieces[Piece::PAWN.0 as usize] |= Bitboard::square_mask(sq_to);
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
        board.bb_pieces[Piece::PAWN.0 as usize] |= Bitboard::square_mask(sq1);
        board.bb_colors[Color::WHITE.index()] |= Bitboard::square_mask(sq1);

        board.pieces[sq2.0 as usize] = Piece::new(Color::WHITE, Piece::KNIGHT);
        board.bb_pieces[Piece::KNIGHT.0 as usize] |= Bitboard::square_mask(sq2);
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
}
