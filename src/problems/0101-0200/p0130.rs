// Created on Sep 30, 2026

use pe_rs::number_theory::{is_prime, multiplicative_order};

pub fn run() {
    let ans: u64 = (5..)
        .filter(|&n| {
            n % 2 != 0
                && n % 5 != 0
                && !is_prime(n)
                && (n - 1) % multiplicative_order(10, 9 * n) == 0
        })
        .take(25)
        .sum();

    println!("{ans}");
}
