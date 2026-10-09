// Created on Oct 08, 2026

use pe_rs::number_theory::is_prime;

fn solve(n: i64) -> i64 {
    let mut ans = 0;

    for n in (10..n).step_by(10) {
        if n % 3 == 0 || !matches!(n % 7, 3 | 4) || !matches!((n * n) % 13, 1 | 3 | 9) {
            continue;
        }
        let ok = [1, 3, 7, 9, 13, 27]
            .into_iter()
            .all(|p| is_prime(n * n + p))
            && !is_prime(n * n + 19)
            && !is_prime(n * n + 21);

        if ok {
            ans += n;
        }
    }
    ans
}

pub fn run() {
    println!("{}", solve(150_000_000));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(1242490, solve(1_000_000));
    }
}
