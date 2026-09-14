# Project Euler with Rust


Benchmarked with:

```shell
cargo build --release --bin [BIN]
hyperfine --warmup 10 --runs 100 --shell=none ./target/release/[BIN]
```

## Problem lists

| Problem                                                                       |     Benchmark     |            Notes             |                                    Code                                     |
|:------------------------------------------------------------------------------|:-----------------:|:----------------------------:|:---------------------------------------------------------------------------:|
| [1005. Median Prime List](https://projecteuler.net/problem=1005)              |  5.8 ms ± 0.3 ms  | DP, combinatorics, unranking | [p1005.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p1005.rs) |
| [20. Factorial Digit Sum](https://projecteuler.net/problem=20)                |  1.9 ms ± 0.2 ms  |                              | [p0020.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0020.rs) |
| [19. Counting Sundays](https://projecteuler.net/problem=19)                   |  1.8 ms ± 0.2 ms  |                              | [p0019.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0019.rs) |
| [18. Maximum Path Sum I](https://projecteuler.net/problem=18)                 |  1.8 ms ± 0.2 ms  |                              | [p0018.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0018.rs) |
| [17. Number Letter Counts](https://projecteuler.net/problem=17)               |  2.1 ms ± 0.2 ms  |                              | [p0017.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0017.rs) |
| [16. Power Digit Sum](https://projecteuler.net/problem=16)                    |  2.6 ms ± 0.3 ms  |                              | [p0016.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0016.rs) |
| [15. Lattice Paths](https://projecteuler.net/problem=15)                      |  2.0 ms ± 0.2 ms  |              DP              | [p0015.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0015.rs) |
| [14. Longest Collatz Sequence](https://projecteuler.net/problem=14)           | 136.2 ms ± 1.0 ms |                              | [p0014.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0014.rs) |
| [13. Large Sum](https://projecteuler.net/problem=13)                          |  2.1 ms ± 0.2 ms  |                              | [p0013.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0013.rs) |
| [12. Highly Divisible Triangular Number](https://projecteuler.net/problem=12) | 14.5 ms ± 0.4 ms  |                              | [p0012.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0012.rs) |
| [11. Largest Product in a Grid](https://projecteuler.net/problem=11)          |  1.9 ms ± 0.2 ms  |                              | [p0011.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0011.rs) |
| [10. Summation of Primes](https://projecteuler.net/problem=10)                |  7.8 ms ± 0.3 ms  |                              | [p0010.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0010.rs) |
| [9. Special Pythagorean Triplet](https://projecteuler.net/problem=9)          |  2.2 ms ± 0.2 ms  |                              | [p0009.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0009.rs) |
| [8. Largest Product in a Series](https://projecteuler.net/problem=8)          |  2.0 ms ± 0.1 ms  |                              | [p0008.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0008.rs) |
| [7. 10 001st Prime](https://projecteuler.net/problem=7)                       |  2.6 ms ± 0.2 ms  |                              | [p0007.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0007.rs) |
| [6. Sum Square Difference](https://projecteuler.net/problem=6)                |  2.0 ms ± 0.1 ms  |                              | [p0006.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0006.rs) |
| [5. Smallest Multiple](https://projecteuler.net/problem=5)                    |  2.0 ms ± 0.2 ms  |                              | [p0005.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0005.rs) |
| [4. Largest Palindrome Product](https://projecteuler.net/problem=4)           | 40.5 ms ± 0.9 ms  |                              | [p0004.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0004.rs) |
| [3. Largest Prime Factor](https://projecteuler.net/problem=3)                 |  1.9 ms ± 0.2 ms  |                              | [p0003.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0003.rs) |
| [2. Even Fibonacci Numbers](https://projecteuler.net/problem=2)               |  2.0 ms ± 0.2 ms  |                              | [p0002.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0002.rs) |
| [1. Multiples of 3 or 5](https://projecteuler.net/problem=1)                  |  2.0 ms ± 0.2 ms  |                              | [p0001.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0001.rs) |
