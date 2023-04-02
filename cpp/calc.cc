#include "calc.h"
#include "types.h"
#include <iostream>

void print_mask(char d[27]) {
  for (int i = 1; i < 27; i++)
    std::cout << (int)(d[i]) << ' ';
  std::cout << std::endl;
}

const Outcome outcome(const char *guess, const char *answer) {
  Outcome outcome = 0;
  char d[27] = {0};

  // check greens
  for (char i = 0; i < 5; i++) {
    if (guess[i] == answer[i]) {
      outcome += GREEN[i];
      d[0] |= 1 << i;
    } else {
      d[answer[i] % 32]++;
    }
  }

  // check yellows
  for (char i = 0; i < 5; i++) {
    if (d[guess[i] % 32] > 0 && (d[0] & 1 << i) == 0) {
      outcome += YELLOW[i];
      d[guess[i] % 32]--;
    }
  }

  return outcome;
}

std::string const outcome_str(Outcome outcome) {
  Outcome x = outcome;
  char resu[6] = {0};
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

const int outcome_test() {
  auto f = [](const std::string g, const std::string a, std::string o) {
    std::string received = outcome_str(outcome(g.c_str(), a.c_str()));
    if (received.compare(o) != 0) {
      std::cerr << "received: " << received << std::endl;
      std::cerr << "expected: " << o << std::endl;
      std::cerr << "Bad comparison" << std::endl;
      return true;
    }
    return false;
  };
  if (f("zzzzz", "xxxxx", "BBBBB")) return 1;
  if (f("zzzzz", "zzzzz", "GGGGG")) return 1;
  if (f("eezzz", "zzzee", "YYGYY")) return 1;
  if (f("adieu", "audio", "GYYBY")) return 1;
  if (f("crust", "rebut", "BYYBG")) return 1;
  if (f("azzzz", "zazzz", "YYGGG")) return 1;
  if (f("azzzz", "zxxxx", "BYBBB")) return 1;
  std::cerr << "[outcome_test] All tests passed!" << std::endl;
  return 0;
}
