use crate::{
    board::{
        Board,
        bitboard::Bitboard,
        defs::{Rank, Square},
    },
    color::Color,
    game::{
        defs::CastlingRights,
        game_move::{Move, MoveFlag},
    },
    movegen::attack_tables::ATTACK_TABLES,
    piece::Piece,
};

pub mod attack_tables;

pub struct Movegen {
    pub board: Board,
}

impl Movegen {
    pub fn new(board: Board) -> Movegen {
        Movegen { board }
    }

    pub fn perft(&self, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }

        let moves = self.legal();

        if depth == 1 {
            return moves.len() as u64;
        }

        let mut nodes = 0;

        for mv in moves {
            let mut next_board = self.board.clone();

            if next_board.make_move(mv).is_ok() {
                let next_movegen = Movegen::new(next_board);
                nodes += next_movegen.perft(depth - 1);
            }
        }
        nodes
    }

    pub fn pseudo_legal(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(64);
        let color = self.board.color_to_move;

        let own_pieces = self.board.color_bb(color.index());
        let occupied = self.board.all_occupied();
        let enemy_pieces = occupied ^ own_pieces;

        self.gen_pawn_moves(&mut moves, occupied, enemy_pieces, color);

        let mut our_knights = self.board.piece_bb(Piece::KNIGHT.index()) & own_pieces;
        while let Some(sq_idx) = our_knights.pop_lsb() {
            let sq = Square(sq_idx);
            let attacks = ATTACK_TABLES.knight(sq) & !own_pieces;
            self.append_moves(&mut moves, sq, attacks, enemy_pieces);
        }

        let mut our_kings = self.board.piece_bb(Piece::KING.index()) & own_pieces;
        if let Some(sq_idx) = our_kings.pop_lsb() {
            let sq = Square(sq_idx);
            let attacks = ATTACK_TABLES.king(sq) & !own_pieces;
            self.append_moves(&mut moves, sq, attacks, enemy_pieces);
        }

        let mut our_bishops = self.board.piece_bb(Piece::BISHOP.index()) & own_pieces;
        while let Some(sq_idx) = our_bishops.pop_lsb() {
            let sq = Square(sq_idx);
            let attacks = self.get_bishop_attacks(sq, occupied) & !own_pieces;
            self.append_moves(&mut moves, sq, attacks, enemy_pieces);
        }

        let mut our_rooks = self.board.piece_bb(Piece::ROOK.index()) & own_pieces;
        while let Some(sq_idx) = our_rooks.pop_lsb() {
            let sq = Square(sq_idx);
            let attacks = self.get_rook_attacks(sq, occupied) & !own_pieces;
            self.append_moves(&mut moves, sq, attacks, enemy_pieces);
        }

        let mut our_queens = self.board.piece_bb(Piece::QUEEN.index()) & own_pieces;
        while let Some(sq_idx) = our_queens.pop_lsb() {
            let sq = Square(sq_idx);
            let attacks = (self.get_bishop_attacks(sq, occupied)
                | self.get_rook_attacks(sq, occupied))
                & !own_pieces;
            self.append_moves(&mut moves, sq, attacks, enemy_pieces);
        }

        self.gen_castling_moves(&mut moves, color);

        moves
    }

    pub fn legal(&self) -> Vec<Move> {
        let candidates = self.pseudo_legal();
        let mut legal_moves = Vec::with_capacity(candidates.len());
        let us = self.board.color_to_move;

        for mv in candidates {
            let mut next_board = self.board.clone();

            if next_board.make_move(mv).is_ok() {
                if !Movegen::new(next_board).is_in_check(us) {
                    legal_moves.push(mv);
                }
            }
        }
        legal_moves
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        let king_bb = self.board.piece_bb(Piece::KING.index()) & self.board.color_bb(color.index());
        let king_sq_idx = match king_bb.lsb() {
            Some(idx) => idx,
            None => return false,
        };

        self.is_square_attacked(Square(king_sq_idx), !color)
    }

    pub fn is_square_attacked(&self, sq: Square, attacker: Color) -> bool {
        let attackers_bb = self.board.color_bb(attacker.index());
        let occupied = self.board.all_occupied();

        let pawn_attacks = ATTACK_TABLES.pawn(sq, !attacker);
        if (pawn_attacks & self.board.piece_bb(Piece::PAWN.index()) & attackers_bb)
            != Bitboard::EMPTY
        {
            return true;
        }

        if (ATTACK_TABLES.knight(sq) & self.board.piece_bb(Piece::KNIGHT.index()) & attackers_bb)
            != Bitboard::EMPTY
        {
            return true;
        }

        if (ATTACK_TABLES.king(sq) & self.board.piece_bb(Piece::KING.index()) & attackers_bb)
            != Bitboard::EMPTY
        {
            return true;
        }

        let rook_like =
            self.board.piece_bb(Piece::ROOK.index()) | self.board.piece_bb(Piece::QUEEN.index());
        if (self.get_rook_attacks(sq, occupied) & rook_like & attackers_bb) != Bitboard::EMPTY {
            return true;
        }

        let bishop_like =
            self.board.piece_bb(Piece::BISHOP.index()) | self.board.piece_bb(Piece::QUEEN.index());
        if (self.get_bishop_attacks(sq, occupied) & bishop_like & attackers_bb) != Bitboard::EMPTY {
            return true;
        }

        false
    }

    fn gen_pawn_moves(
        &self,
        moves: &mut Vec<Move>,
        occupied: Bitboard,
        enemy: Bitboard,
        color: Color,
    ) {
        let mut pawns =
            self.board.piece_bb(Piece::PAWN.index()) & self.board.color_bb(color.index());
        if pawns == Bitboard::EMPTY {
            return;
        }

        let empty = !occupied;
        let ep_square_opt = self.board.en_passant_square;

        let push_promotions = |moves: &mut Vec<Move>, src: Square, dest: Square| {
            moves.push(Move::new(MoveFlag::PROMOTION_QUEEN, src, dest));
            moves.push(Move::new(MoveFlag::PROMOTION_ROOK, src, dest));
            moves.push(Move::new(MoveFlag::PROMOTION_BISHOP, src, dest));
            moves.push(Move::new(MoveFlag::PROMOTION_KNIGHT, src, dest));
        };

        while let Some(src_idx) = pawns.pop_lsb() {
            let src = Square(src_idx);
            let src_mask = Bitboard::square_mask(src);

            if color == Color::WHITE {
                let one_bb = src_mask.north_one();
                if one_bb != Bitboard::EMPTY && (one_bb & empty) != Bitboard::EMPTY {
                    let dest = Square(one_bb.lsb().unwrap());

                    if dest.get_rank() == Rank::EIGHT {
                        push_promotions(moves, src, dest);
                    } else {
                        moves.push(Move::new(MoveFlag::NONE, src, dest));

                        if (src_mask & Bitboard::rank_mask(Rank::TWO)) != Bitboard::EMPTY {
                            let two_bb = one_bb.north_one();
                            if two_bb != Bitboard::EMPTY && (two_bb & empty) != Bitboard::EMPTY {
                                let dest2 = Square(two_bb.lsb().unwrap());
                                moves.push(Move::new(MoveFlag::NONE, src, dest2));
                            }
                        }
                    }
                }

                let mut caps = (src_mask.north_east_one() | src_mask.north_west_one()) & enemy;
                while let Some(dest_idx) = caps.pop_lsb() {
                    let dest = Square(dest_idx);
                    if dest.get_rank() == Rank::EIGHT {
                        push_promotions(moves, src, dest);
                    } else {
                        moves.push(Move::new(MoveFlag::CAPTURE, src, dest));
                    }
                }

                if let Some(ep_sq) = ep_square_opt {
                    let ep_mask = Bitboard::square_mask(ep_sq);
                    if (src_mask.north_east_one() & ep_mask) != Bitboard::EMPTY
                        || (src_mask.north_west_one() & ep_mask) != Bitboard::EMPTY
                    {
                        let capture_sq = Square(ep_sq.0 - 8);
                        let cap_mask = Bitboard::square_mask(capture_sq);
                        if (cap_mask & self.board.piece_bb(Piece::PAWN.index()) & enemy)
                            != Bitboard::EMPTY
                        {
                            moves.push(Move::new(MoveFlag::EN_PASSANT, src, ep_sq));
                        }
                    }
                }
            } else {
                let one_bb = src_mask.south_one();
                if one_bb != Bitboard::EMPTY && (one_bb & empty) != Bitboard::EMPTY {
                    let dest = Square(one_bb.lsb().unwrap());

                    if dest.get_rank() == Rank::ONE {
                        push_promotions(moves, src, dest);
                    } else {
                        moves.push(Move::new(MoveFlag::NONE, src, dest));

                        if (src_mask & Bitboard::rank_mask(Rank::SEVEN)) != Bitboard::EMPTY {
                            let two_bb = one_bb.south_one();
                            if two_bb != Bitboard::EMPTY && (two_bb & empty) != Bitboard::EMPTY {
                                let dest2 = Square(two_bb.lsb().unwrap());
                                moves.push(Move::new(MoveFlag::NONE, src, dest2));
                            }
                        }
                    }
                }

                let mut caps = (src_mask.south_east_one() | src_mask.south_west_one()) & enemy;
                while let Some(dest_idx) = caps.pop_lsb() {
                    let dest = Square(dest_idx);
                    if dest.get_rank() == Rank::ONE {
                        push_promotions(moves, src, dest);
                    } else {
                        moves.push(Move::new(MoveFlag::CAPTURE, src, dest));
                    }
                }

                if let Some(ep_sq) = ep_square_opt {
                    let ep_mask = Bitboard::square_mask(ep_sq);
                    if (src_mask.south_east_one() & ep_mask) != Bitboard::EMPTY
                        || (src_mask.south_west_one() & ep_mask) != Bitboard::EMPTY
                    {
                        let capture_sq = Square(ep_sq.0 + 8);
                        let cap_mask = Bitboard::square_mask(capture_sq);
                        if (cap_mask & self.board.piece_bb(Piece::PAWN.index()) & enemy)
                            != Bitboard::EMPTY
                        {
                            moves.push(Move::new(MoveFlag::EN_PASSANT, src, ep_sq));
                        }
                    }
                }
            }
        }
    }

    #[inline]
    fn gen_castling_moves(&self, moves: &mut Vec<Move>, color: Color) {
        if color == Color::WHITE {
            if (self.board.castling_rights & CastlingRights::WK) != CastlingRights::NONE {
                if !self.board.is_square_occupied(Square::F1)
                    && !self.board.is_square_occupied(Square::G1)
                    && !self.is_square_attacked(Square::E1, Color::BLACK)
                    && !self.is_square_attacked(Square::F1, Color::BLACK)
                    && !self.is_square_attacked(Square::G1, Color::BLACK)
                {
                    moves.push(Move::new(MoveFlag::CASTLING, Square::E1, Square::G1));
                }
            }
            if (self.board.castling_rights & CastlingRights::WQ) != CastlingRights::NONE {
                if !self.board.is_square_occupied(Square::D1)
                    && !self.board.is_square_occupied(Square::C1)
                    && !self.board.is_square_occupied(Square::B1)
                    && !self.is_square_attacked(Square::E1, Color::BLACK)
                    && !self.is_square_attacked(Square::D1, Color::BLACK)
                    && !self.is_square_attacked(Square::C1, Color::BLACK)
                {
                    moves.push(Move::new(MoveFlag::CASTLING, Square::E1, Square::C1));
                }
            }
        } else {
            if (self.board.castling_rights & CastlingRights::BK) != CastlingRights::NONE {
                if !self.board.is_square_occupied(Square::F8)
                    && !self.board.is_square_occupied(Square::G8)
                    && !self.is_square_attacked(Square::E8, Color::WHITE)
                    && !self.is_square_attacked(Square::F8, Color::WHITE)
                    && !self.is_square_attacked(Square::G8, Color::WHITE)
                {
                    moves.push(Move::new(MoveFlag::CASTLING, Square::E8, Square::G8));
                }
            }
            if (self.board.castling_rights & CastlingRights::BQ) != CastlingRights::NONE {
                if !self.board.is_square_occupied(Square::D8)
                    && !self.board.is_square_occupied(Square::C8)
                    && !self.board.is_square_occupied(Square::B8)
                    && !self.is_square_attacked(Square::E8, Color::WHITE)
                    && !self.is_square_attacked(Square::D8, Color::WHITE)
                    && !self.is_square_attacked(Square::C8, Color::WHITE)
                {
                    moves.push(Move::new(MoveFlag::CASTLING, Square::E8, Square::C8));
                }
            }
        }
    }

    #[inline]
    fn append_moves(
        &self,
        moves: &mut Vec<Move>,
        src: Square,
        mut attacks: Bitboard,
        enemies: Bitboard,
    ) {
        while let Some(dest_idx) = attacks.pop_lsb() {
            let dest = Square(dest_idx);
            let is_capture = (Bitboard::square_mask(dest) & enemies) != Bitboard::EMPTY;
            let flag = if is_capture {
                MoveFlag::CAPTURE
            } else {
                MoveFlag::NONE
            };
            moves.push(Move::new(flag, src, dest));
        }
    }

    fn get_bishop_attacks(&self, sq: Square, occ: Bitboard) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;

        let mut b = Bitboard::square_mask(sq);
        while {
            b = b.north_east_one();
            b != Bitboard::EMPTY
        } {
            attacks |= b;
            if (b & occ) != Bitboard::EMPTY {
                break;
            }
        }

        b = Bitboard::square_mask(sq);
        while {
            b = b.north_west_one();
            b != Bitboard::EMPTY
        } {
            attacks |= b;
            if (b & occ) != Bitboard::EMPTY {
                break;
            }
        }

        b = Bitboard::square_mask(sq);
        while {
            b = b.south_east_one();
            b != Bitboard::EMPTY
        } {
            attacks |= b;
            if (b & occ) != Bitboard::EMPTY {
                break;
            }
        }

        b = Bitboard::square_mask(sq);
        while {
            b = b.south_west_one();
            b != Bitboard::EMPTY
        } {
            attacks |= b;
            if (b & occ) != Bitboard::EMPTY {
                break;
            }
        }
        attacks
    }

    fn get_rook_attacks(&self, sq: Square, occ: Bitboard) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;

        let mut b = Bitboard::square_mask(sq);
        while {
            b = b.north_one();
            b != Bitboard::EMPTY
        } {
            attacks |= b;
            if (b & occ) != Bitboard::EMPTY {
                break;
            }
        }

        b = Bitboard::square_mask(sq);
        while {
            b = b.south_one();
            b != Bitboard::EMPTY
        } {
            attacks |= b;
            if (b & occ) != Bitboard::EMPTY {
                break;
            }
        }

        b = Bitboard::square_mask(sq);
        while {
            b = Bitboard((b.0 << 1) & Bitboard::NOT_A_FILE);
            b != Bitboard::EMPTY
        } {
            attacks |= b;
            if (b & occ) != Bitboard::EMPTY {
                break;
            }
        }

        b = Bitboard::square_mask(sq);
        while {
            b = Bitboard((b.0 >> 1) & Bitboard::NOT_H_FILE);
            b != Bitboard::EMPTY
        } {
            attacks |= b;
            if (b & occ) != Bitboard::EMPTY {
                break;
            }
        }
        attacks
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use crate::{board::Board, movegen::Movegen};

    const PERFT_EXPECTED_COUNTS: [u64; 8] =
        [1, 20, 400, 8902, 197281, 4865609, 119060324, 3195901860];

    #[test]
    fn test_perft() {
        println!("test game::movegen::test_perft ... starting perft");
        let board =
            Board::from_fen(Board::START_FEN).expect("Failed to load starting board from FEN");

        let movegen = Movegen::new(board);

        for ply in 1..PERFT_EXPECTED_COUNTS.len() {
            let expected = PERFT_EXPECTED_COUNTS[ply];
            let start = Instant::now();
            let nodes = movegen.perft(ply as u8);
            let end = Instant::now();
            let time: Duration = end - start;

            if nodes == expected {
                println!(
                    "test game::movegen::test_perft::ply::{} ({}/{}) {}ms ... \x1b[92m ok \x1b[0m",
                    ply,
                    nodes,
                    expected,
                    time.as_millis(),
                );
            } else {
                println!(
                    "test game::movegen::test_perft::ply::{} ... ({}/{}) {}ms \x1b[31m FAILURE \x1b[0m",
                    ply,
                    nodes,
                    expected,
                    time.as_millis(),
                );
            }
        }
    }
}
