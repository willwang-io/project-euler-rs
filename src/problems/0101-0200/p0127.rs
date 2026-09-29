// Created on Sep 28, 2026

use pe_rs::number_theory::gcd;

fn solve(n: usize) -> usize {
    let mut rad_sieve = vec![1u64; n + 1];
    for i in 2..=n {
        if rad_sieve[i] != 1 {
            continue;
        }
        for j in (i..=n).step_by(i) {
            rad_sieve[j] *= i as u64;
        }
    }

    let mut cand: Vec<usize> = (1..n).collect();
    cand.sort_unstable_by_key(|&a| rad_sieve[a]);

    let mut ans = 0;
    for c in 2..n {
        if rad_sieve[c] == c as u64 {
            continue;
        }
        for &a in &cand {
            let tmp = rad_sieve[a] * rad_sieve[c];
            if tmp >= c as u64 {
                break;
            }
            if a > (c - 1) / 2 {
                continue;
            }
            if tmp * rad_sieve[c - a] < c as u64 && gcd(a as u64, c as u64) == 1 {
                ans += c;
            }
        }
    }
    ans
}

pub fn run() {
    println!("{}", solve(120_000));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(12523, solve(1000));
    }
}
