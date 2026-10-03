// Created on Oct 03, 2026

use pe_rs::number_theory::sieve_of_eratosthenes;

pub fn run() {
    let lmt = 50_000_000;
    let is_prime = sieve_of_eratosthenes(lmt);
    let mut ans = 2;

    for p in (3..lmt).step_by(2) {
        if !is_prime[p] {
            continue;
        }
        if p % 4 == 3 {
            ans += 1;
        }
        if p <= (lmt - 1) / 4 {
            ans += 1;
        }
        if p <= (lmt - 1) / 16 {
            ans += 1;
        }
    }

    println!("{ans}");
}
