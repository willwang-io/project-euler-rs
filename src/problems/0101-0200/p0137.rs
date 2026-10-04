// Created on Oct 04, 2026

pub fn run() {
    let mut x = 1u64;
    let mut y = 1u64;
    let n = 15;
    for _ in 1..n * 2 {
        (x, y) = (y, x + y);
    }
    println!("{}", (x * y) / (y * y - x * y - x * x));
}
