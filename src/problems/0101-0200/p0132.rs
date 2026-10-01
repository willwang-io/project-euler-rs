// Created on Oct 01, 2026

use pe_rs::number_theory::{multiplicative_order, sieve_of_eratosthenes};

fn r(n: u64) -> usize {
    let lmt = 1_000_001;
    let is_prime = sieve_of_eratosthenes(lmt);
    let ans: usize = (7..)
        .filter(|&p| is_prime[p] && n % multiplicative_order(10, p as u64) == 0)
        .take(40)
        .sum();
    ans
}

pub fn run() {
    println!("{}", r(1_000_000_000));
}
