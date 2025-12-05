use crate::{
    board::{
        bitboard::Bitboard,
        defs::{File, NrOf, Rank, Square},
    },
    color::Color,
    game::{
        defs::CastlingRights,
        game_move::{Move, MoveFlag},
    },
    piece::Piece,
};

pub mod bitboard;
pub mod defs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    pub bb_pieces: [Bitboard; NrOf::PIECES],
    pub bb_colors: [Bitboard; NrOf::COLORS],
    pub pieces: [Piece; NrOf::SQUARES],
    pub color_to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
}
impl Board {
    pub const EMPTY: Board = Board {
        bb_pieces: [Bitboard::EMPTY; NrOf::PIECES],
        bb_colors: [Bitboard::EMPTY; NrOf::COLORS],
        pieces: [Piece::NONE; NrOf::SQUARES],
        color_to_move: Color::WHITE,
        castling_rights: CastlingRights::NONE,
        en_passant_square: None,
        halfmove_clock: 0,
        fullmove_number: 1,
    };

    pub const START_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    pub const STARTING_BOARD: Board = Board {
        bb_colors: [Bitboard(0xFFFF), Bitboard(0xFFFF000000000000)],
        bb_pieces: [
            Bitboard(0xFF00000000FF00),
            Bitboard(0x4200000000000042),
            Bitboard(0x2400000000000024),
            Bitboard(0x8100000000000081),
            Bitboard(0x800000000000008),
            Bitboard(0x1000000000000010),
        ],
        pieces: [
            Piece::new(Color::WHITE, Piece::ROOK),
            Piece::new(Color::WHITE, Piece::KNIGHT),
            Piece::new(Color::WHITE, Piece::BISHOP),
            Piece::new(Color::WHITE, Piece::QUEEN),
            Piece::new(Color::WHITE, Piece::KING),
            Piece::new(Color::WHITE, Piece::BISHOP),
            Piece::new(Color::WHITE, Piece::KNIGHT),
            Piece::new(Color::WHITE, Piece::ROOK),
            Piece::new(Color::WHITE, Piece::PAWN),
            Piece::new(Color::WHITE, Piece::PAWN),
            Piece::new(Color::WHITE, Piece::PAWN),
            Piece::new(Color::WHITE, Piece::PAWN),
            Piece::new(Color::WHITE, Piece::PAWN),
            Piece::new(Color::WHITE, Piece::PAWN),
            Piece::new(Color::WHITE, Piece::PAWN),
            Piece::new(Color::WHITE, Piece::PAWN),
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::NONE,
            Piece::new(Color::BLACK, Piece::PAWN),
            Piece::new(Color::BLACK, Piece::PAWN),
            Piece::new(Color::BLACK, Piece::PAWN),
            Piece::new(Color::BLACK, Piece::PAWN),
            Piece::new(Color::BLACK, Piece::PAWN),
            Piece::new(Color::BLACK, Piece::PAWN),
            Piece::new(Color::BLACK, Piece::PAWN),
            Piece::new(Color::BLACK, Piece::PAWN),
            Piece::new(Color::BLACK, Piece::ROOK),
            Piece::new(Color::BLACK, Piece::KNIGHT),
            Piece::new(Color::BLACK, Piece::BISHOP),
            Piece::new(Color::BLACK, Piece::QUEEN),
            Piece::new(Color::BLACK, Piece::KING),
            Piece::new(Color::BLACK, Piece::BISHOP),
            Piece::new(Color::BLACK, Piece::KNIGHT),
            Piece::new(Color::BLACK, Piece::ROOK),
        ],
        castling_rights: CastlingRights::ALL,
        color_to_move: Color::WHITE,
        en_passant_square: None,
        halfmove_clock: 0,
        fullmove_number: 1,
    };
}

impl Board {
    pub fn from_fen(fen: &str) -> Result<Board, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 6 {
            return Err(format!(
                "FEN string must have 6 parts. Found {}",
                parts.len()
            ));
        }

        let mut board = Board::EMPTY;

        let placement = parts[0];
        let mut rank = 7;
        let mut file = 0;

        for c in placement.chars() {
            if c == '/' {
                rank -= 1;
                file = 0;
            } else if let Some(digit) = c.to_digit(10) {
                file += digit as usize;
            } else {
                if rank < 0 || file >= 8 {
                    return Err(format!(
                        "FEN parsing error: Invalid piece position at rank {} file {}",
                        rank + 1,
                        file
                    ));
                }

                let color = if c.is_uppercase() {
                    Color::WHITE
                } else {
                    Color::BLACK
                };
                let piece_type = match c.to_ascii_lowercase() {
                    'p' => Piece::PAWN,
                    'n' => Piece::KNIGHT,
                    'b' => Piece::BISHOP,
                    'r' => Piece::ROOK,
                    'q' => Piece::QUEEN,
                    'k' => Piece::KING,
                    _ => return Err(format!("Invalid piece character: {}", c)),
                };

                let piece = Piece::new(color, piece_type);
                let square = Square::from_rank_file(Rank(rank as u8), File(file as u8));
                let sq_idx = square.0 as usize;

                board.pieces[sq_idx] = piece;
                board.bb_pieces[piece_type.index()] |= Bitboard::square_mask(square);
                board.bb_colors[color.index()] |= Bitboard::square_mask(square);

                file += 1;
            }
        }

        board.color_to_move = match parts[1] {
            "w" => Color::WHITE,
            "b" => Color::BLACK,
            _ => return Err("Invalid active color in FEN".to_string()),
        };

        let castle_str = parts[2];
        if castle_str != "-" {
            for c in castle_str.chars() {
                match c {
                    'K' => board.castling_rights |= CastlingRights::WK,
                    'Q' => board.castling_rights |= CastlingRights::WQ,
                    'k' => board.castling_rights |= CastlingRights::BK,
                    'q' => board.castling_rights |= CastlingRights::BQ,
                    _ => return Err(format!("Invalid castling char: {}", c)),
                }
            }
        }

        let ep_str = parts[3];
        if ep_str != "-" {
            if ep_str.len() != 2 {
                return Err("Invalid En Passant square format".to_string());
            }
            let chars: Vec<char> = ep_str.chars().collect();
            let file_char = chars[0];
            let rank_char = chars[1];

            let file_idx = (file_char as u8).wrapping_sub(b'a');
            let rank_idx = (rank_char as u8).wrapping_sub(b'1');

            if file_idx > 7 || rank_idx > 7 {
                return Err("En Passant square out of bounds".to_string());
            }

            if (board.color_to_move == Color::WHITE && rank_idx != 2)
                || (board.color_to_move == Color::BLACK && rank_idx != 5)
            {
                return Err("En Passant square not on correct rank for side to move".to_string());
            }

            board.en_passant_square = Some(Square::from_rank_file(Rank(rank_idx), File(file_idx)));
        }

        board.halfmove_clock = parts[4]
            .parse::<u8>()
            .map_err(|_| "Invalid halfmove clock")?;

        board.fullmove_number = parts[5]
            .parse::<u16>()
            .map_err(|_| "Invalid fullmove number")?;

        Ok(board)
    }
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
    fn make_capture_move(
        &mut self,
        dest_index: usize,
        dest_mask: Bitboard,
    ) -> Result<(), MakeMoveError> {
        let captured_piece = self.pieces[dest_index];
        if captured_piece == Piece::NONE {
            return Err(MakeMoveError::NoCaptureVictim);
        }

        if captured_piece.get_type() == Piece::ROOK {
            let sq = Square(dest_index as u8);
            match sq {
                Square::A1 => self.castling_rights &= !CastlingRights::WQ,
                Square::H1 => self.castling_rights &= !CastlingRights::WK,
                Square::A8 => self.castling_rights &= !CastlingRights::BQ,
                Square::H8 => self.castling_rights &= !CastlingRights::BK,
                _ => {}
            }
        }

        let captured_piece_index = captured_piece.index();
        let captured_color_index = captured_piece
            .get_color()
            .ok_or(MakeMoveError::InvalidPieceColor)?
            .index();

        self.bb_pieces[captured_piece_index] &= !dest_mask;
        self.bb_colors[captured_color_index] &= !dest_mask;
        self.pieces[dest_index] = Piece::NONE;

        Ok(())
    }

    fn make_en_passant_move(&mut self, dest: Square, color: Color) -> Result<(), MakeMoveError> {
        let capture_square = if color == Color::WHITE {
            Square(dest.0 - 8)
        } else {
            Square(dest.0 + 8)
        };

        let capture_index = capture_square.0 as usize;
        let captured_piece = self.pieces[capture_index];

        if captured_piece == Piece::NONE {
            return Err(MakeMoveError::NoCaptureVictim);
        }

        let captured_type = captured_piece.index();
        let captured_color_index = captured_piece
            .get_color()
            .ok_or(MakeMoveError::InvalidPieceColor)?
            .index();

        let mask = Bitboard::square_mask(capture_square);
        self.bb_pieces[captured_type] &= !mask;
        self.bb_colors[captured_color_index] &= !mask;
        self.pieces[capture_index] = Piece::NONE;

        self.en_passant_square = None;

        Ok(())
    }

    fn make_castling_move(&mut self, dest: Square, color_index: usize) {
        let (rook_src, rook_dest) = match dest {
            Square::G1 => (Square::H1, Square::F1),
            Square::C1 => (Square::A1, Square::D1),
            Square::G8 => (Square::H8, Square::F8),
            Square::C8 => (Square::A8, Square::D8),
            _ => unreachable!("Invalid castling move"),
        };

        let rook_piece = self.pieces[rook_src.0 as usize];
        let rook_piece_index = rook_piece.index();

        let src_mask = Bitboard::square_mask(rook_src);
        let dest_mask = Bitboard::square_mask(rook_dest);

        self.bb_pieces[rook_piece_index] &= !src_mask;
        self.bb_colors[color_index] &= !src_mask;
        self.bb_pieces[rook_piece_index] |= dest_mask;
        self.bb_colors[color_index] |= dest_mask;

        self.pieces[rook_dest.0 as usize] = rook_piece;
        self.pieces[rook_src.0 as usize] = Piece::NONE;

        if color_index == Color::WHITE.index() {
            self.castling_rights &= !(CastlingRights::WK | CastlingRights::WQ);
        } else {
            self.castling_rights &= !(CastlingRights::BK | CastlingRights::BQ);
        }

        self.en_passant_square = None;
    }

    fn make_promotion_move(
        &mut self,
        src: Square,
        dest: Square,
        dest_index: usize,
        color: Color,
        color_index: usize,
        flag: MoveFlag,
    ) {
        let pawn_piece = self.pieces[src.0 as usize];
        let pawn_piece_index = pawn_piece.index();

        let src_mask = Bitboard::square_mask(src);
        self.bb_pieces[pawn_piece_index] &= !src_mask;
        self.bb_colors[color_index] &= !src_mask;
        self.pieces[src.0 as usize] = Piece::NONE;

        let dest_mask = Bitboard::square_mask(dest);
        let dest_piece = self.pieces[dest_index];
        if dest_piece != Piece::NONE {
            let dest_piece_index = dest_piece.index();
            let dest_color_index = dest_piece
                .get_color()
                .expect("captured piece must have color")
                .index();

            if dest_piece.get_type() == Piece::ROOK {
                match dest {
                    Square::A1 => self.castling_rights &= !CastlingRights::WQ,
                    Square::H1 => self.castling_rights &= !CastlingRights::WK,
                    Square::A8 => self.castling_rights &= !CastlingRights::BQ,
                    Square::H8 => self.castling_rights &= !CastlingRights::BK,
                    _ => {}
                }
            }

            self.bb_pieces[dest_piece_index] &= !dest_mask;
            self.bb_colors[dest_color_index] &= !dest_mask;
            self.pieces[dest_index] = Piece::NONE;
        }

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

        self.en_passant_square = None;
    }

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

        if moving_piece.get_type() == Piece::KING {
            if color == Color::WHITE {
                self.castling_rights &= !(CastlingRights::WK | CastlingRights::WQ);
            } else {
                self.castling_rights &= !(CastlingRights::BK | CastlingRights::BQ);
            }
        } else if moving_piece.get_type() == Piece::ROOK {
            match src {
                Square::A1 => self.castling_rights &= !CastlingRights::WQ,
                Square::H1 => self.castling_rights &= !CastlingRights::WK,
                Square::A8 => self.castling_rights &= !CastlingRights::BQ,
                Square::H8 => self.castling_rights &= !CastlingRights::BK,
                _ => {}
            }
        }

        match flag {
            MoveFlag::CAPTURE => {
                // Remove captured piece and clear en-passant target
                self.make_capture_move(dest_index, dest_mask)?;
                self.en_passant_square = None;
            }
            MoveFlag::EN_PASSANT => {
                self.make_en_passant_move(dest, color)?;
                // make_en_passant_move already sets en_passant_square = None, but keep explicit for clarity:
                self.en_passant_square = None;
            }
            MoveFlag::CASTLING => {
                self.make_castling_move(dest, color_index);
                // make_castling_move already clears EP, but keep explicit for clarity:
                self.en_passant_square = None;
            }
            MoveFlag::PROMOTION_BISHOP
            | MoveFlag::PROMOTION_KNIGHT
            | MoveFlag::PROMOTION_ROOK
            | MoveFlag::PROMOTION_QUEEN => {
                self.make_promotion_move(src, dest, dest_index, color, color_index, flag);
                self.en_passant_square = None;
                self.color_to_move = !self.color_to_move;
                return Ok(());
            }

            MoveFlag::NONE => {
                if moving_piece.get_type() == Piece::PAWN {
                    if color == Color::WHITE {
                        if src.get_rank() == Rank::TWO && dest.get_rank() == Rank::FOUR {
                            self.en_passant_square = Some(Square(src.0 + 8));
                        } else {
                            self.en_passant_square = None;
                        }
                    } else {
                        if src.get_rank() == Rank::SEVEN && dest.get_rank() == Rank::FIVE {
                            self.en_passant_square = Some(Square(src.0 - 8));
                        } else {
                            self.en_passant_square = None;
                        }
                    }
                } else {
                    self.en_passant_square = None;
                }
            }
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

        let sq1 = Square::A1;
        let sq2 = Square::H8;

        board.bb_pieces[Piece::PAWN.index()] = Bitboard::square_mask(sq1);
        board.bb_pieces[Piece::KNIGHT.index()] = Bitboard::square_mask(sq2);

        board.bb_colors[Piece::PAWN.index()] = Bitboard::square_mask(sq1);
        board.bb_colors[Piece::KNIGHT.index()] = Bitboard::square_mask(sq2);

        let occupied = board.all_occupied();

        assert!(board.is_square_occupied(sq1));
        assert!(board.is_square_occupied(sq2));

        let expected = Bitboard::square_mask(sq1) | Bitboard::square_mask(sq2);
        assert_eq!(occupied, expected);
    }

    #[test]
    fn test_simple_move() {
        let mut board = Board::EMPTY;

        let sq_from = Square::A1;
        let sq_to = Square::B1;

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

        let sq_from = Square::A1;
        let sq_to = Square::B1;
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

        let sq1 = Square::A1;
        let sq2 = Square::B1;
        board.pieces[sq1.0 as usize] = Piece::new(Color::WHITE, Piece::PAWN);
        board.bb_pieces[Piece::PAWN.index()] |= Bitboard::square_mask(sq1);
        board.bb_colors[Color::WHITE.index()] |= Bitboard::square_mask(sq1);

        board.pieces[sq2.0 as usize] = Piece::new(Color::WHITE, Piece::KNIGHT);
        board.bb_pieces[Piece::KNIGHT.index()] |= Bitboard::square_mask(sq2);
        board.bb_colors[Color::WHITE.index()] |= Bitboard::square_mask(sq2);

        let mv = Move::new(MoveFlag::NONE, sq1, Square::C1);
        board.make_move(mv).unwrap();

        assert_eq!(board.pieces[sq1.0 as usize], Piece::NONE);
        assert_eq!(
            board.pieces[Square::C1.0 as usize],
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

        let wp_sq = Square::E4;
        board.pieces[wp_sq.0 as usize] = Piece::new(Color::WHITE, Piece::PAWN);
        board.bb_pieces[Piece::PAWN.index()] |= Bitboard::square_mask(wp_sq);
        board.bb_colors[Color::WHITE.index()] |= Bitboard::square_mask(wp_sq);

        let bp_sq = Square::D4;
        board.pieces[bp_sq.0 as usize] = Piece::new(Color::BLACK, Piece::PAWN);
        board.bb_pieces[Piece::PAWN.index()] |= Bitboard::square_mask(bp_sq);
        board.bb_colors[Color::BLACK.index()] |= Bitboard::square_mask(bp_sq);

        let ep_move = Move::new(MoveFlag::EN_PASSANT, wp_sq, Square::D5);
        board.make_move(ep_move).expect("En passant should succeed");

        assert_eq!(board.pieces[bp_sq.0 as usize], Piece::NONE);
        assert_eq!(
            board.bb_colors[Color::BLACK.index()] & Bitboard::square_mask(bp_sq),
            Bitboard::EMPTY
        );

        assert_eq!(
            board.pieces[Square::D5.0 as usize],
            Piece::new(Color::WHITE, Piece::PAWN)
        );
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

    #[test]
    fn from_fen() {
        let board = Board::from_fen(Board::START_FEN).unwrap();

        assert_eq!(board, Board::STARTING_BOARD);
    }
}
