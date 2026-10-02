// Created on Oct 01, 2026

use pe_rs::number_theory::{multiplicative_order, sieve_of_eratosthenes};

pub fn run() {
    let lmt = 100_000;
    let mut is_prime = sieve_of_eratosthenes(lmt);

    for p in 7..lmt {
        if is_prime[p] {
            let mut g = multiplicative_order(10, p as u64);
            while g % 2 == 0 {
                g /= 2;
            }
            while g % 5 == 0 {
                g /= 5;
            }
            if g == 1 {
                is_prime[p] = false;
            }
        }
    }

    let ans: usize = (2..lmt).filter(|&i| is_prime[i]).sum();
    println!("{ans}");
}
