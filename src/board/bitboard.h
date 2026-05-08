#include <stdint.h>
#include <stdbool.h>

typedef uint64_t Bitboard;

typedef enum  {
    BITBOARD_OK = 0,
    BITBOARD_ERR = 1
} BitboardResult;

bool get_square(Bitboard *bb, uint8_t sq);
BitboardResult from_square(uint8_t sq, Bitboard* bb);
