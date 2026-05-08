#include <assert.h>
#include "board/bitboard.h"

int main() {
    Bitboard bb;
    assert(from_square(0, &bb) == BITBOARD_OK);
    assert(bb == 0b1);
    assert(from_square(63, &bb) == BITBOARD_OK);
    assert(bb == 0x8000000000000000);

    return 0;
}
