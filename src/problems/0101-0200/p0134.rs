// Created on Oct 02, 2026

use pe_rs::number_theory::{mod_pow, sieve_of_eratosthenes};

pub fn run() {
    let lmt = 1_000_004;
    let is_prime = sieve_of_eratosthenes(lmt);
    let primes: Vec<_> = (5..lmt).filter(|&i| is_prime[i]).collect();

    let mut ans = 0;
    for w in primes.windows(2) {
        let p1 = w[0] as u64;
        let p2 = w[1] as u64;
        let k = p1.to_string().len() as u64;

        let inv = mod_pow(mod_pow(10, k, p2), p2 - 2, p2);
        let t = (p2 - p1) * inv % p2;
        let s = 10u64.pow(k as u32) * t + p1;
        ans += s;
    }
    println!("{ans}");
}
