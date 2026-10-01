// Created on Sep 30, 2026

// n^3 + n^2p = n^2(n + p). In order this to be a cube, we must have n = a^3 and n + p = b^3.
// Thus, a^3 + p = b^3 => p = b^3 - a^3 = (b - a)(b^2 + ab + a^2). Since p is prime, so b - a = 1.
// Substitute b = a + 1 gives p = (a + 1)^3 - a^3 = 3a(a + 1) + 1.

use pe_rs::number_theory::sieve_of_eratosthenes;

pub fn run() {
    let lmt = 1_000_000;
    let is_prime = sieve_of_eratosthenes(lmt);

    let mut ans = 0;
    for a in 1.. {
        let p = 3 * a * (a + 1) + 1;
        if p >= lmt {
            break;
        }
        if is_prime[p] {
            ans += 1;
        }
    }
    println!("{ans}");
}
