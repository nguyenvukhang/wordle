#include "calc.h"
#include "words.h"
#include <exception>
#include <iostream>
#include <string>

int main(int argc, char *argv[]) {
  outcome_test();
  int x = 0;
  for (auto guess : GUESSES) {
    for (auto answer : ANSWERS) {
      Outcome out = outcome(guess.c_str(), answer.c_str());
      x += out;
    }
  }
  std::cout << x << std::endl;
  return 0;
}
