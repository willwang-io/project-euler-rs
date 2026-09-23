use std::collections::HashSet;

fn digit_factorial_sum(mut n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    let mut ans = 0u64;
    while n > 0 {
        ans += (1..=n % 10).product::<u64>();
        n /= 10;
    }
    ans
}

fn chain_len(mut n: u64, seen: &mut HashSet<u64>) -> usize {
    seen.clear();
    while seen.insert(n) {
        n = digit_factorial_sum(n);
    }
    seen.len()
}

pub fn run() {
    let mut seen = HashSet::new();
    let mut ans = 0;
    for i in 0..1_000_000 {
        if chain_len(i, &mut seen) == 60 {
            ans += 1;
        }
    }
    println!("{ans}");
}
