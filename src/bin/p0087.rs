use pe_rs::number_theory::sieve_of_eratosthenes;

fn main() {
    let limit = 50_000_000usize;
    let is_prime = sieve_of_eratosthenes(limit.isqrt() + 1);
    let mut cnt = vec![0u64; limit / 64 + 1];

    for c in 2..=85 {
        if !is_prime[c] {
            continue;
        }
        for b in 2..=369 {
            if !is_prime[b] {
                continue;
            }
            let total = c * c * c * c + b * b * b;
            if total >= limit {
                break;
            }
            let upper = limit - total;
            for a in 2..=upper.isqrt() {
                if !is_prime[a] {
                    continue;
                }
                let tmp = a * a + total;
                cnt[tmp / 64] |= 1 << (tmp % 64);
            }
        }
    }

    let ans: usize = cnt.iter().map(|word| word.count_ones() as usize).sum();
    println!("{ans}");
}
