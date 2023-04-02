#include "calc.h"
#include "words.h"
#include <chrono>
#include <exception>
#include <iostream>
#include <string>

void bench() {
  auto guesses = Words::GUESSES;
  auto answers = Words::ANSWERS;
  long x = 1;

  auto start = std::chrono::high_resolution_clock::now();
  for (int g = 0; g < Words::GUESS_COUNT; g++) {
    for (int a = 0; a < Words::ANSWER_COUNT; a++) {
      x *= 2;
      x %= 100000007;
      // x++;
    }
  }
  auto stop = std::chrono::high_resolution_clock::now();

  std::cout << x << std::endl;
  std::cout << std::chrono::duration<double, std::milli>(stop - start).count()
            << std::endl;
}

int main(int argc, char *argv[]) {
  if (outcome_test() == 1)
    return 0;
  bench();
  return 0;
}
