# Wordle Solver

### Algorithm

At any given state, the answer list is a subset of a (known) complete
list, filtered based on previous guesses and observed outcomes.

The next guess is determined by the expected information gained, given
that list of answers remaining.

$$
\text{Expected Information}=\sum_x P(x)\cdot \log_2\left(\frac{1}{P(x)}\right)
$$

where $x$ denotes a particular outcome when setting a guess.

Of course, we can only obtain $P(x)$ because we already know the
complete list of answers beforehand.

### Runtime

Brute-forcing this problem is expensive, since there are **12,974**
possible guesses and **2,309** possible answers (a lower-bound of
**29,956,966** comparisons when finding calculating the expected
information of all guesses).

This project optimizes runtime by caching previously calculated values
of expected information gain.

The information gain function requires two things:

1. A list of all possible guesses
2. A list of remaining viable answers

Since the list of remaining answers can have 2^2309 possible states,
it's not feasible to hash it as a state.

The implemented cache operates differently. It's based on the
observation that the best guess is solely determined by the "path"
taken to get to the current game state.

Take this for example. Having played a game like this in the past:

```
(correct answer is "rebut")
"soare" -> BBBYY -> "direr" -> BBYYB -> "crust" -> "BBYYG" -> "rebut"
```

If we were to encounter a mid-game state like this again:

```
"soare" -> BBBYY -> "direr" -> BBYYB -> (? to play)
```

The clear choice is to play `"crust"` next, since there is no
difference in information received from the first game.

And so the cache takes the form of a (directed, acyclic) graph, where
each node looks like this:

```rust
struct Node {
    next: Vec<(Outcome, Node)>,
    guess: Option<Word>,
}
```

Using this, and other optimizations (using `u8` to store outcomes –
since there are 3⁵ = 243 different outcomes, and `u8` stores up to 255
different states), the run-time achieved stands at **1.95s** on a
consumer laptop:

```
$ cargo build --release
   Compiling wordle v0.1.0 (/Users/khang/repos/wordle)
    Finished release [optimized] target(s) in 0.57s

$ ./target/release/wordle
230/2309 (1.135475791s)
460/2309 (215.518125ms)
690/2309 (187.301916ms)
920/2309 (131.187333ms)
1150/2309 (84.023208ms)
1380/2309 (74.534375ms)
1610/2309 (36.892041ms)
1840/2309 (44.16675ms)
2070/2309 (23.506791ms)
2300/2309 (8.521666ms)
time elapsed: 1.941474583s
avg tries: 3.6327414465136423
```

(Note the acceleration in solving speed as the cache warms up. Each
duration value is the duration since the last mark.)
