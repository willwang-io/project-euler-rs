// Created on Sep 27, 2026

use pe_rs::utility::is_palindrome;
use std::collections::HashSet;

pub fn run() {
    let mut p_sum = vec![0];
    let mut cur = 0;

    for i in 1.. {
        cur += i * i;
        if cur >= 100_000_000_000 {
            break;
        }
        p_sum.push(cur);
    }

    let mut seen = HashSet::new();
    for i in 2..p_sum.len() {
        for j in 0..i - 1 {
            let sum: u64 = p_sum[i] - p_sum[j];
            if sum < 100_000_000 && is_palindrome(&format!("{sum}")) {
                seen.insert(sum);
            }
        }
    }
    println!("{}", seen.iter().sum::<u64>());
}
