#ifndef WORDLE_TYPES_H
#define WORDLE_TYPES_H

#include <stdint.h>

using Outcome = uint8_t;
using Word = char[6];

const uint8_t YELLOW[]{81, 27, 9, 3, 1};
const uint8_t GREEN[]{162, 54, 18, 6, 2};

#endif
