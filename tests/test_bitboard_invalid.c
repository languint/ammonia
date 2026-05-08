#include <assert.h>
#include "board/bitboard.h"

int main() {
    Bitboard bb;
    assert(from_square(64, &bb) == BITBOARD_ERR);
}
