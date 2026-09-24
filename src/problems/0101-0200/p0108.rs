fn solve(limit: u64) -> u64 {
    fn dfs(primes: &[u64], max_exp: u64, mut n: u64, cnt: u64, limit: u64, best: &mut u64) {
        if cnt > limit {
            *best = (*best).min(n);
            return;
        }
        let p = primes[0];
        for e in 1..=max_exp {
            if n > (*best - 1) / p {
                break;
            }
            n *= p;
            dfs(&primes[1..], e, n, cnt * (2 * e + 1), limit, best);
        }
    }

    let mut best = u64::MAX;
    dfs(&[2, 3, 5, 7, 11, 13, 17, 19], 10, 1, 1, limit, &mut best);
    best
}

pub fn run() {
    let ans = solve(1000 * 2 - 1);
    println!("{}", ans);
}
