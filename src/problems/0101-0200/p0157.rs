// Created on Nov 01, 2026

use pe_rs::number_theory::{divisors, gcd};

fn solve(n: u32) -> usize {
    let cur = 10i64.pow(n);
    let mut ans = 0;

    for d in divisors(cur) {
        for x in 1..=d.isqrt() {
            if d % x == 0 {
                let y = d / x;
                if gcd(x, y) == 1 {
                    let g = cur * (x + y) / (x * y);
                    ans += divisors(g).len();
                }
            }
        }
    }

    ans
}

pub fn run() {
    let ans = (1..=9).fold(0, |acc, x| acc + solve(x));
    println!("{ans}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(20, solve(1));
    }
}
