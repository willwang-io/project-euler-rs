// Created on Nov 06, 2026

pub fn run() {
    let n = 1_000_000;

    let mut dp: Vec<_> = (0..=n)
        .map(|i| if i == 0 { 0 } else { 1 + (i - 1) % 9 })
        .collect();

    for b in 2..=n / 2 {
        for a in 2..=b.min(n / b) {
            dp[a * b] = dp[a * b].max(dp[a] + dp[b]);
        }
    }

    let ans: usize = dp[2..n].iter().sum();
    println!("{ans}");
}
