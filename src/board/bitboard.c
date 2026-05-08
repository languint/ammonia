#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "bitboard.h"

bool get_square(Bitboard* bb, uint8_t sq) {
    if (sq >= 64) {
        fprintf(stderr, "Bitboard::get_square(): sq out of bounds: %u\n", sq);
        exit(EXIT_FAILURE);
    }

    return (*bb & (uint64_t)(1ULL << sq)) != 0;
}

BitboardResult from_square(uint8_t sq, Bitboard* bb) {
    if (sq >= 64) {
        fprintf(stderr, "Bitboard::from_square(): sq out of bounds: %u\n", sq);
        return BITBOARD_ERR;
    }

    *bb = (uint64_t)(1ULL << sq);

    return BITBOARD_OK;
}
