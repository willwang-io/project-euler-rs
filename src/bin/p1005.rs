// Use DP to count the increasing prime lists that sum to N, including the number of valid
// completion for a given prefix.
//
// For instance, `dfs(0, 2026)` counts all the valid lists, while `dfs(1, 2026 - 2)` counts the
// valid lists whose first prime is 2.
//
// We can then examine prefixes in lexicographic order, skip entire groups using these counts, and
// locate the median list.
//
// Sep 13, 2026

use pe_rs::number_theory::sieve_of_eratosthenes;

const N: usize = 2026;

fn main() {
    let s = sieve_of_eratosthenes(N);
    let p = (2..N).filter(|&i| s[i]).collect::<Vec<usize>>();

    fn dfs(i: usize, sum: usize, p: &[usize], memo: &mut [Vec<i64>]) -> i64 {
        if i == p.len() {
            return 0;
        }
        if memo[i][sum] != -1 {
            return memo[i][sum];
        }
        if sum == 0 {
            return 1;
        }
        if p[i] > sum {
            return 0;
        }
        let ans = dfs(i + 1, sum - p[i], p, memo) + dfs(i + 1, sum, p, memo);
        memo[i][sum] = ans;
        ans
    }

    let mut memo = vec![vec![-1i64; N + 1]; p.len()];
    let total = dfs(0, N, &p, &mut memo);
    let mut k = total / 2;
    let mut remaining = N;

    let mut ans = 1;

    for (i, &x) in p.iter().enumerate() {
        if x > remaining {
            break;
        }
        let cnt = dfs(i + 1, remaining - x, &p, &mut memo);
        if cnt < k {
            k -= cnt;
        } else {
            ans = (ans * x) % 1_000_000_000;
            remaining -= x;
        }
    }

    println!("{}", ans);
}
