#include "words.h"
#include <iostream>

int main(int argc, char *argv[]) {
  std::cout << "hello" << std::endl;
  for (auto i : guesses) {
    std::cout << i << std::endl;
  }
  return 0;
}
