# Wordle Solver

### Algorithm

This algorithm is greedy. At any given point, the answer list is a
subset of a (known) complete list, filtered based on previous guesses
and observed outcomes.

The next guess is determined by the expected information gained given
the list of answers remaining.

$$
\text{Expected Information}=\sum_x P(x)\cdot \log_2\left(\frac{1}{P(x)}\right)
$$

where $x$ denotes a particular outcome when setting a guess.

Of course, we can only obtain $P(x)$ because we already know the
complete list of answers beforehand.

### Running the main binary

Running the generated binary gets an average rating of how many
guesses are required to solve.

General approach:

1. Fix an answer.
2. Run the algorithm to solve.
3. Track the number of guesses required and take average.

### Runtime

Brute-forcing this problem will take a good amount of time, since
there are 12,974 possible guesses and 2,309 possible answers, leading to
a minimum of 12,974 × 2,309 = 29,956,966 comparisons when finding the
highest entropy of the first guess.

This project optimizies runtime by caching previously known
calculations of best entropy.

Best entropy is a calculation that requires these input:

1. A list of all possible guesses
2. A list of remaining viable answers

So it's pretty hard to make a hash table cache, since the list of
remaining answers can technically have 2^2309 possible states.

The cache operates on the observation that the best guess is solely
determined by the "path" taken to get to the current game state.

Take this for example. Having played a game like this in the past:

```
(correct answer is "rebut")
"soare" -> BBBYY -> "direr" -> BBYYB -> "crust" -> "BBYYG" -> "rebut"
```

If we were to encounter, mid-game, a history like this again:

```
"soare" -> BBBYY -> "direr" -> BBYYB -> (? to play)
```

The clear choice is to play `"crust"` next, since there is no
difference in information received from the first game, and the
calculations have already been ran.

And so the cache takes the form of a (directed, acyclic) graph, where
each node looks like this:

```rust
struct Node {
    next: HashMap<Outcome, Node>,
    guess: Option<Word>,
}
```

Using this, and other optimizations such as using a `u8` to store an
`Outcome` (since there are 3^5 = 243 different outcomes, and `u8`
variables have up to 255 different states), the end run-time achieved
stands at 1.97s on a consumer laptop:

```
230/2309 (1.170911375s)
460/2309 (218.2595ms)
690/2309 (187.797291ms)
920/2309 (135.617166ms)
1150/2309 (85.500166ms)
1380/2309 (73.044625ms)
1610/2309 (33.966083ms)
1840/2309 (46.0555ms)
2070/2309 (24.23325ms)
2300/2309 (8.62225ms)
time elapsed: 1.984464875s
avg tries: 3.6327414465136423
```

(Note the acceleration in solving speed as the cache warms up. Each
duration value is the duration since the last mark.)
