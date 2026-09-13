# Project Euler with Rust

| Problem                                                          |     Benchmark      |             Tags             |                                    Code                                     |
|:-----------------------------------------------------------------|:------------------:|:----------------------------:|:---------------------------------------------------------------------------:|
| [1005. Median Prime List](https://projecteuler.net/problem=1005) | 🟢 5.8 ms ± 0.3 ms | DP, combinatorics, unranking | [p1005.rs](https://github.com/willwang-io/pe-rs/blob/main/src/bin/p1005.rs) |

### Benchmarked with:
```shell
cargo build --release --bin [EXE]
hyperfine --warmup 10 --runs 100 --shell=none ./target/release[EXE]
```
