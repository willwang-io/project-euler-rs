# Project Euler with Rust


### Benchmarked with:
```shell
cargo build --release --bin [EXE]
hyperfine --warmup 10 --runs 100 --shell=none ./target/release/[EXE]
```

| Problem                                                              |    Benchmark     |            Notes             |                                    Code                                     |
|:---------------------------------------------------------------------|:----------------:|:----------------------------:|:---------------------------------------------------------------------------:|
| [1005. Median Prime List](https://projecteuler.net/problem=1005)     | 5.8 ms ± 0.3 ms  | DP, combinatorics, unranking | [p1005.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p1005.rs) |
| [10. Summation of Primes](https://projecteuler.net/problem=10)       | 7.8 ms ± 0.3 ms  |                              | [p0010.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0010.rs) |
| [9. Special Pythagorean Triplet](https://projecteuler.net/problem=9) | 2.2 ms ± 0.2 ms  |                              | [p0009.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0009.rs) |
| [8. Largest Product in a Series](https://projecteuler.net/problem=8) | 2.0 ms ± 0.1 ms  |                              | [p0008.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0008.rs) |
| [7. 10 001st Prime](https://projecteuler.net/problem=7)              | 2.6 ms ± 0.2 ms  |                              | [p0007.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0007.rs) |
| [6. Sum Square Difference](https://projecteuler.net/problem=6)       | 2.0 ms ± 0.1 ms  |                              | [p0006.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0006.rs) |
| [5. Smallest Multiple](https://projecteuler.net/problem=5)           | 2.0 ms ± 0.2 ms  |                              | [p0005.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0005.rs) |
| [4. Largest Palindrome Product](https://projecteuler.net/problem=4)  | 40.5 ms ± 0.9 ms |                              | [p0004.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0004.rs) |
| [3. Largest Prime Factor](https://projecteuler.net/problem=3)        | 1.9 ms ± 0.2 ms  |                              | [p0003.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0003.rs) |
| [2. Even Fibonacci Numbers](https://projecteuler.net/problem=2)      | 2.0 ms ± 0.2 ms  |                              | [p0002.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0002.rs) |
| [1. Multiples of 3 or 5](https://projecteuler.net/problem=1)         | 2.0 ms ± 0.2 ms  |                              | [p0001.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p0001.rs) |
