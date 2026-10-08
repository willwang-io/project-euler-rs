// Created on Oct 07, 2026

use pe_rs::number_theory::gcd;

pub fn run() {
    let lmt = 120_000usize;
    let mut adj = vec![vec![]; lmt + 1];

    for m in 2..=(3 * lmt).isqrt() {
        for n in 1..m {
            if gcd(m as i64, n as i64) != 1 {
                continue;
            }
            let mut a = m * m - n * n;
            let mut b = 2 * m * n + n * n;
            let c = m * m + m * n + n * n;
            let g = gcd(gcd(a as i64, b as i64), c as i64) as usize;

            a /= g;
            b /= g;

            if a > b {
                (a, b) = (b, a);
            }

            if a + b > lmt {
                continue;
            }

            for k in 1..=lmt / (a + b) {
                adj[k * a].push(k * b);
            }
        }
    }

    for tmp in &mut adj {
        tmp.sort_unstable();
        tmp.dedup();
    }

    let mut seen = vec![false; lmt + 1];
    let mut ans = 0;

    for p in 1..=lmt {
        for &q in &adj[p] {
            for &r in &adj[q] {
                let sum = p + q + r;
                if sum > lmt {
                    break;
                }
                if adj[p].binary_search(&r).is_ok() && !seen[sum] {
                    seen[sum] = true;
                    ans += sum;
                }
            }
        }
    }

    println!("{ans}");
}
