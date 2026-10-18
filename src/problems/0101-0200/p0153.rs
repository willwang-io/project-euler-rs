// Created on Oct 17, 2026

use pe_rs::number_theory::gcd;

fn solve(n: i64) -> i64 {
    let mut ans = 0;

    // Sum of rationals divisors.
    for d in 1..=n {
        ans += d * (n / d);
    }

    for x in 1..n {
        if 2 * x * x > n {
            break;
        }

        for y in x..n {
            let q = x * x + y * y;

            if q > n {
                break;
            }

            if gcd(x, y) != 1 {
                continue;
            }

            let a0 = if x == y { x } else { x + y };
            let m = n / q;
            let mut sum = 0;

            for g in 1..=m {
                sum += 2 * a0 * g * (m / g);
            }

            ans += sum;
        }
    }

    ans
}

pub fn run() {
    println!("{}", solve(100_000_000));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(17924657155, solve(100_000));
    }
}
