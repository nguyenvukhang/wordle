#include "calc.h"
#include "words.h"
#include <chrono>
#include <exception>
#include <iostream>
#include <string>

int main(int argc, char *argv[]) {
  outcome_test();
  int x = 0;
  auto start = std::chrono::high_resolution_clock::now();
  const char **guess = Words::GUESSES;
  const char **answer = Words::ANSWERS;

  for (int g = 0; g < Words::GUESS_COUNT; g++, guess++) {
    answer = Words::ANSWERS;
    for (int a = 0; a < Words::ANSWER_COUNT; a++, answer++) {
      Outcome out = outcome(*guess, *answer);
      x += out;
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
