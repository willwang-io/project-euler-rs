/*
Created on Oct 12, 2026

For each prime, the filter looks at numbers divisible by its highest remaining power. It checks
which subsets meet the required remainder modulo p^2 and removes any number that appears in none
of them. Once nothing else can be removed, the search recursively includes or skips each remaining
candidate.
 */

use pe_rs::number_theory::{lcm, mod_pow, sieve_of_eratosthenes};

fn filter_candidates(n: usize) -> Vec<i64> {
    let mut possible = vec![true; n + 1];
    let is_prime = sieve_of_eratosthenes(n + 1);

    loop {
        let mut changed = false;

        for p in 2..=n {
            if !is_prime[p] {
                continue;
            }

            for e in (1..8).rev() {
                let cur = p.pow(e);
                let g: Vec<_> = (2..=n).filter(|&j| j % cur == 0 && possible[j]).collect();

                if g.is_empty() {
                    continue;
                }

                let modulus = (p * p) as i64;
                let residues: Vec<_> = g
                    .iter()
                    .map(|&j| mod_pow((j / cur).pow(2) as i64, (p * (p - 1) - 1) as i64, modulus))
                    .collect();
                let mut keep = vec![false; g.len()];

                for mask in 1..(1 << g.len()) {
                    let mut sum = 0;
                    for j in 0..g.len() {
                        if mask & (1 << j) != 0 {
                            sum = (sum + residues[j]) % modulus;
                        }
                    }

                    if sum == if p == 2 && e == 1 { 2 } else { 0 } {
                        for j in 0..g.len() {
                            if mask & (1 << j) != 0 {
                                keep[j] = true;
                            }
                        }
                    }
                }

                for j in 0..g.len() {
                    if !keep[j] {
                        possible[g[j]] = false;
                        changed = true;
                    }
                }

                break;
            }
        }

        if !changed {
            break;
        }
    }

    (2..=n)
        .filter(|&i| possible[i])
        .map(|x| x as i64)
        .collect::<Vec<_>>()
}

fn solve(n: usize) -> i64 {
    fn dfs(i: usize, need: i64, available: i64, a: &[i64]) -> i64 {
        if need == 0 {
            return 1;
        }
        if i == a.len() || need < 0 || need > available {
            return 0;
        }

        let rest = available - a[i];
        dfs(i + 1, need, rest, a) + dfs(i + 1, need - a[i], rest, a)
    }

    let candidates = filter_candidates(n);
    let l = candidates.iter().fold(1, |acc, &x| lcm(acc, x));
    let weighs: Vec<_> = candidates.iter().map(|&x| (l / x).pow(2)).collect();

    dfs(0, l * l / 2, weighs.iter().sum(), &weighs)
}

pub fn run() {
    println!("{}", solve(80));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(3, solve(45));
    }
}
