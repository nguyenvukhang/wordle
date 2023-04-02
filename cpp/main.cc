#include "calc.h"
// #include "words.h"
#include <exception>
#include <iostream>
#include <string>

int check(const std::string r, const std::string e) {
  if (r.compare(e) != 0) {
    std::cout << "received: " << r << std::endl;
    std::cout << "expected: " << e << std::endl;
    std::cout << "Bad comparison" << std::endl;
    return 1;
  }
  return 0;
}

int main(int argc, char *argv[]) {
  std::cout << "hello" << std::endl;

  auto f = [](const std::string g, const std::string a, std::string o) {
    return check(outcome_str(outcome(g.c_str(), a.c_str())), o);
  };

  if (f("zzzzz", "xxxxx", "BBBBB") == 1) return 0;
  if (f("zzzzz", "zzzzz", "GGGGG") == 1) return 0;
  if (f("eezzz", "zzzee", "YYGYY") == 1) return 0;
  if (f("adieu", "audio", "GYYBY") == 1) return 0;
  if (f("crust", "rebut", "BYYBG") == 1) return 0;
  if (f("azzzz", "zazzz", "YYGGG") == 1) return 0;
  if (f("azzzz", "zxxxx", "BYBBB") == 1) return 0;
  return 0;
}
