// Created on Oct 04, 2026

// After brute-forced a few, it looks like the relationship is: A_{n} = 18 * A_{n-1} - A_{n - 2}
// with A_0 = 1 and A_1 = 17.
// 17
// 305
// 5473
// 98209
// 1762289

pub fn run() {
    let mut x = 1u64;
    let mut y = 17u64;
    let mut ans = 0;
    for _ in 0..12 {
        ans += y;
        (x, y) = (y, y * 18 - x);
    }
    println!("{ans}");
}
