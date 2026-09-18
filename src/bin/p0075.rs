use pe_rs::number_theory::gcd;

fn main() {
    let limit = 1_500_000;
    let mut cnt = vec![0; limit + 1];
    for m in 1..limit.isqrt() {
        for n in 1..m {
            if gcd(m as u64, n as u64) != 1 || (m + n) % 2 == 0 {
                continue;
            }
            let a = m * m - n * n;
            let b = 2 * m * n;
            let c = m * m + n * n;
            let mut k = 1;
            while k * (a + b + c) <= limit {
                cnt[(a + b + c) * k] += 1;
                k += 1;
            }
        }
    }
    let ans = (1..=limit).filter(|&x| cnt[x] == 1).count();
    println!("{ans}");
}
