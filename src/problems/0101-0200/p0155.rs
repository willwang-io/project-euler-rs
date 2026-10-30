// Created on Oct 19, 2026

use pe_rs::number_theory::gcd;
use std::collections::{HashMap, HashSet};

fn solve(n: usize) -> i64 {
    fn dfs(n: usize, memo: &mut HashMap<usize, HashSet<(i64, i64)>>) -> HashSet<(i64, i64)> {
        if n == 1 {
            return HashSet::from([(1, 1)]);
        }

        if let Some(ans) = memo.get(&n) {
            return ans.clone();
        }

        let mut ans = HashSet::new();

        for i in 1..=n / 2 {
            let left = dfs(i, memo);
            let right = dfs(n - i, memo);

            for &(p, q) in &left {
                for &(r, s) in &right {
                    for (a, b) in [(p * s + r * q, q * s), (p * r, p * s + r * q)] {
                        let g = gcd(a, b);
                        ans.insert((a / g, b / g));
                    }
                }
            }
        }

        memo.insert(n, ans.clone());
        ans
    }

    let mut seen = HashSet::new();
    let mut memo = HashMap::new();

    for i in 1..=n {
        seen.extend(dfs(i, &mut memo));
    }
    seen.len() as _
}

pub fn run() {
    let ans = solve(18);
    println!("{ans}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(1, solve(1));
        assert_eq!(3, solve(2));
        assert_eq!(7, solve(3));
    }
}
