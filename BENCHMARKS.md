# Benching Wordle

Here's some experimental turns that this project took.

1. [Loops: Rust vs C++](#loops%3A-rust-vs-c%2B%2B)

### Loops: Rust vs C++

I went into this expecting C++ to run faster than Rust by a small
margin, since it is less smart about memory management.

In both languages, I implemented a loop that will compute all possible
outcomes of all pairs of guesses and answers, and output the integer
sum of those outcomes.

<details>
<summary>Rust</summary>

```rust
// main loop
fn bench() {
    let answers = words::answers();
    let guesses = words::guesses();
    let mut x = 0usize;

    let start = Instant::now();
    for a in &answers {
        for g in &guesses {
            x += outcome(g, a) as usize
        }
    }
    println!("{x}");
    println!("{:?}", Instant::elapsed(&start));
}

// calculate outcome
fn outcome(guess: &[u8; 5], answer: &[u8; 5]) -> u8 {
    let (mut outcome, mut d) = (0, [0u8; 27]);
    for i in 0..5 {
        if guess[i] == answer[i] {
            outcome += GREEN[i];
            d[0] |= 1 << i;
        } else {
            d[answer[i] % 32 as usize] += 1;
        }
    }
    for i in 0..5 {
        if d[guess[i] % 32 as usize] > 0 && d[0] & 1 << i == 0 {
            outcome += YELLOW[i];
            d[guess[i] % 32 as usize] -= 1;
        }
    }
    outcome
}
```

</details>

<details>
<summary>C++</summary>

```cpp
// main loop
void bench() {
  auto guesses = Words::GUESSES;
  auto answers = Words::ANSWERS;
  int x = 0;

  auto start = std::chrono::high_resolution_clock::now();
  for (int g = 0; g < Words::GUESS_COUNT; g++) {
    for (int a = 0; a < Words::ANSWER_COUNT; a++) {
      x += outcome(guesses[g], answers[a]);
    }
  }
  auto stop = std::chrono::high_resolution_clock::now();

  std::cout << x << std::endl;
  std::cout << std::chrono::duration<double, std::milli>(stop - start).count()
            << std::endl;
}

// calculate outcome
const uint8_t outcome(const char *guess, const char *answer) {
  uint8_t outcome = 0;
  char d[27] = {0};
  for (int i = 0; i < 5; i++) {
    if (guess[i] == answer[i]) {
      outcome += GREEN[i];
      d[0] |= 1 << i;
    } else {
      d[answer[i] % 32]++;
    }
  }
  for (int i = 0; i < 5; i++) {
    if (d[guess[i] % 32] > 0 && (d[0] & 1 << i) == 0) {
      outcome += YELLOW[i];
      d[guess[i] % 32]--;
    }
  }
  return outcome;
}

```

</details>

I ran the benchmarks a few times to arrive at a stable average, and
here's what I got for each:

```sh
$ ./rust_bench
1053874769
275.413958ms

$ ./cpp_bench
1053874769
391.742
```

Even with the lack of proper sampling, the fact that the Rust
implementation runs 40% faster than the C++ one is surprising.

To try to narrow down, I isolated the loop to do a simpler
calculation:

```rust
// rust loop
fn bench() {
    let answers = words::answers();
    let guesses = words::guesses();
    let mut x = 0usize;

    let start = Instant::now();
    for a in &answers {
        for g in &guesses {
            x *= 2;
            x %= 100000007;
        }
    }
    println!("{x}");
    println!("{:?}", Instant::elapsed(&start));
}
```

```cpp
// c++ loop
void bench() {
  auto guesses = Words::GUESSES;
  auto answers = Words::ANSWERS;
  int x = 0;

  auto start = std::chrono::high_resolution_clock::now();
  for (int g = 0; g < Words::GUESS_COUNT; g++) {
    for (int a = 0; a < Words::ANSWER_COUNT; a++) {
      x *= 2;
      x %= 100000007;
    }
  }
  auto stop = std::chrono::high_resolution_clock::now();

  std::cout << x << std::endl;
  std::cout << std::chrono::duration<double, std::milli>(stop - start).count()
            << std::endl;
}

```

And the results seem more likely: Rust runs at about 90% of C++'s
speed.

```
$ ./rust_bench
19511054
114.509083ms

$ ./cpp_bench
19511054
102.535
```
