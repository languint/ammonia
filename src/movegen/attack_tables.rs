use crate::{
    board::{bitboard::Bitboard, defs::Square},
    color::Color,
};

pub struct AttackTables {
    knight_attacks: [Bitboard; 64],
    king_attacks: [Bitboard; 64],

    white_pawn_attacks: [Bitboard; 64],
    black_pawn_attacks: [Bitboard; 64],
}

impl AttackTables {
    pub const fn new() -> Self {
        let mut tables = AttackTables {
            knight_attacks: [Bitboard::EMPTY; 64],
            king_attacks: [Bitboard::EMPTY; 64],
            white_pawn_attacks: [Bitboard::EMPTY; 64],
            black_pawn_attacks: [Bitboard::EMPTY; 64],
        };

        let mut sq = 0;
        while sq < 64 {
            tables.knight_attacks[sq] = Self::compute_knight_attacks(sq as u8);
            tables.king_attacks[sq] = Self::compute_king_attacks(sq as u8);
            tables.white_pawn_attacks[sq] = Self::compute_white_pawn_attacks(sq as u8);
            tables.black_pawn_attacks[sq] = Self::compute_black_pawn_attacks(sq as u8);
            sq += 1;
        }

        tables
    }

    const fn compute_knight_attacks(sq: u8) -> Bitboard {
        let b = 1u64 << sq;
        let mut attacks = 0u64;

        attacks |= (b << 17) & Bitboard::NOT_A_FILE;
        attacks |= (b << 15) & Bitboard::NOT_H_FILE;
        attacks |= (b << 10) & Bitboard::NOT_AB_FILE;
        attacks |= (b << 6) & Bitboard::NOT_GH_FILE;
        attacks |= (b >> 17) & Bitboard::NOT_H_FILE;
        attacks |= (b >> 15) & Bitboard::NOT_A_FILE;
        attacks |= (b >> 10) & Bitboard::NOT_GH_FILE;
        attacks |= (b >> 6) & Bitboard::NOT_AB_FILE;

        Bitboard(attacks)
    }

    const fn compute_king_attacks(sq: u8) -> Bitboard {
        let b = 1u64 << sq;
        let mut attacks = 0u64;

        attacks |= (b << 8);
        attacks |= (b >> 8);
        attacks |= (b << 9) & Bitboard::NOT_A_FILE;
        attacks |= (b << 7) & Bitboard::NOT_H_FILE;
        attacks |= (b >> 7) & Bitboard::NOT_A_FILE;
        attacks |= (b >> 9) & Bitboard::NOT_H_FILE;
        attacks |= (b << 1) & Bitboard::NOT_A_FILE;
        attacks |= (b >> 1) & Bitboard::NOT_H_FILE;

        Bitboard(attacks)
    }

    const fn compute_white_pawn_attacks(sq: u8) -> Bitboard {
        let b = 1u64 << sq;
        let mut attacks = 0u64;

        attacks |= (b << 9) & Bitboard::NOT_A_FILE;
        attacks |= (b << 7) & Bitboard::NOT_H_FILE;

        Bitboard(attacks)
    }

    const fn compute_black_pawn_attacks(sq: u8) -> Bitboard {
        let b = 1u64 << sq;
        let mut attacks = 0u64;

        attacks |= (b >> 7) & Bitboard::NOT_A_FILE;
        attacks |= (b >> 9) & Bitboard::NOT_H_FILE;

        Bitboard(attacks)
    }

    #[inline]
    pub fn knight(&self, sq: Square) -> Bitboard {
        self.knight_attacks[sq.0 as usize]
    }

    #[inline]
    pub fn king(&self, sq: Square) -> Bitboard {
        self.king_attacks[sq.0 as usize]
    }

    #[inline]
    pub fn pawn(&self, sq: Square, color: Color) -> Bitboard {
        if color == Color::WHITE {
            self.white_pawn_attacks[sq.0 as usize]
        } else {
            self.black_pawn_attacks[sq.0 as usize]
        }
    }
}

pub static ATTACK_TABLES: AttackTables = AttackTables::new();
