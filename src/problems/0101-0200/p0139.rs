// Created on Oct 04, 2026

pub fn run() {
    let lmt = 100_000_000u64;
    let mut ans = 0;
    for m in 1..lmt.isqrt() {
        for n in 1..m {
            let perimeter = 2 * m * (m + n);
            if perimeter > lmt {
                break;
            }
            if (m + n) % 2 == 0 {
                continue;
            }
            if (m * m - n * n).abs_diff(2 * m * n) == 1 {
                ans += lmt / perimeter;
            }
        }
    }
    println!("{ans}");
}
