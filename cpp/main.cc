#include "calc.h"
#include "words.h"
#include <chrono>
#include <exception>
#include <iostream>
#include <string>

int main(int argc, char *argv[]) {
  if (outcome_test() == 1) return 0;
  int x = 0;
  auto start = std::chrono::high_resolution_clock::now();
  auto guesses = Words::GUESSES;
  auto answers = Words::ANSWERS;

  for (int g = 0; g < Words::GUESS_COUNT; g++) {
    for (int a = 0; a < Words::ANSWER_COUNT; a++) {
      x += outcome(guesses[g], answers[a]);
      // x++;
    }
  }

  auto stop = std::chrono::high_resolution_clock::now();
  auto diff = stop - start;
  std::cout << "result: " << x << std::endl;
  std::cout << "elapsed: "
            << std::chrono::duration<double, std::milli>(diff).count()
            << std::endl;
  return 0;
}
