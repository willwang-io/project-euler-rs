// Created on Oct 05, 2026

use pe_rs::number_theory::gcd;

fn solve(n: u128) -> u128 {
    let mut ans = 0;

    for q in 1..=n.isqrt().isqrt() {
        'p_loop: for p in q + 1..n {
            if gcd(p as i64, q as i64) != 1 {
                continue;
            }
            for k in 1..n {
                let a = k * q * q;
                let b = k * p * q;
                let c = k * p * p;

                let y = b * c + a;

                if y >= n {
                    if k == 1 {
                        break 'p_loop;
                    }
                    break;
                }

                if y.isqrt() * y.isqrt() == y {
                    ans += y;
                }
            }
        }
    }

    ans
}

pub fn run() {
    let ans = solve(1_000_000_000_000);
    println!("{ans}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(124657, solve(100_000));
    }
}
