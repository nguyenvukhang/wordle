#ifndef WORDLE_WORDS_H
#define WORDLE_WORDS_H

#include <string>
#include <vector>

class Words {
public:
  static const char GUESSES[][6];
  static const char ANSWERS[][6];
  static const int GUESS_COUNT;
  static const int ANSWER_COUNT;
};

#endif
