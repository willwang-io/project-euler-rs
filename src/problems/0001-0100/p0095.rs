use std::collections::HashSet;

fn chain_len(mut n: usize, divisor_sum: &[usize]) -> usize {
    let mut seen = HashSet::new();
    let start = n;
    while n != 0 && n <= divisor_sum.len() {
        if !seen.insert(n) {
            return if n == start { seen.len() } else { 0 };
        }
        n = divisor_sum[n];
    }
    0
}

pub fn run() {
    let limit = 1_000_000;
    let mut divisor_sum_sieve = vec![0usize; limit + 1];
    for d in 1..=limit / 2 {
        for m in (2 * d..=limit).step_by(d) {
            divisor_sum_sieve[m] += d;
        }
    }

    let mut longest = 0;
    let mut ans = 0;
    for i in 1..=limit {
        let cur_len = chain_len(i, &divisor_sum_sieve);
        if cur_len > longest {
            longest = cur_len;
            ans = i;
        }
    }
    println!("{ans}");
}
