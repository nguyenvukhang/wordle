#ifndef WORDLE_CALC_H
#define WORDLE_CALC_H

#include "types.h"

#include <string>

Outcome const outcome(const char *guess, const char *answer);
std::string const outcome_str(Outcome);

#endif
