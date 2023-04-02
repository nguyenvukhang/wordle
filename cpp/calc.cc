#include "calc.h"
#include "types.h"
#include <iostream>

Outcome const outcome(const char *guess, const char *answer) {
  Outcome outcome = 0;
  char d[27] = {0};

  // check greens
  for (int i = 0; i < 5; i++, guess++, answer++) {
    if (*guess == *answer) {
      outcome += GREEN[i];
      d[0] |= 1 << i;
    } else {
      d[*answer % 32]++;
    }
  }
  guess -= 5;

  // check yellows
  for (int i = 0; i < 5; i++, guess++) {
    if (d[*guess % 32] > 0 && (d[0] & 1 << i) == 0) {
      outcome += YELLOW[i];
      d[*guess % 32]--;
    }
  }

  return outcome;
}

std::string const outcome_str(Outcome outcome) {
  Outcome x = outcome;
  char resu[5];
  for (int i = 0; i < 5; i++) {
    switch (x % 3) {
    case 2:
      resu[4 - i] = 'G';
      break;
    case 1:
      resu[4 - i] = 'Y';
      break;
    default:
      resu[4 - i] = 'B';
      break;
    }
    x /= 3;
  }
  return std::string(resu);
}
