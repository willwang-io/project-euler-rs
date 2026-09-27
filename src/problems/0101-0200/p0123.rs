// Created on Sep 27, 2026

use pe_rs::number_theory::sieve_of_eratosthenes;

pub fn run() {
    let is_prime = sieve_of_eratosthenes(1_000_000);
    let primes: Vec<_> = (2..1_000_000).filter(|&i| is_prime[i]).collect();

    for (i, &p) in primes.iter().enumerate() {
        if i % 2 == 0 && (2 * (i + 1) * p) % (p * p) > 10_000_000_000 {
            println!("{}", i + 1);
            break;
        }
    }
}
