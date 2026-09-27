// Created on Sep 27, 2026

use pe_rs::number_theory::prime_factors;

fn rad(n: u64) -> u64 {
    let prime_factors = prime_factors(n);
    prime_factors.iter().map(|(p, _)| p).product()
}

pub fn run() {
    let mut arr: Vec<_> = (1..=100_000).map(|i| (i, rad(i))).collect();
    arr.sort_by(|a, b| a.1.cmp(&b.1));
    println!("{:?}", arr[10_000 - 1].0);
}
