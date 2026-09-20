fn main() {
    let mut p = vec![1];
    const MOD: i64 = 1_000_000;

    loop {
        let n = p.len();
        let mut total = 0i64;
        let mut k = 1;
        loop {
            let a = k * (3 * k - 1) / 2;
            if a > n {
                break;
            }
            let sign = if k % 2 == 1 { 1 } else { -1 };
            total = (total + sign * p[n - a]) % MOD;
            let b = k * (3 * k + 1) / 2;
            if b <= n {
                total = (total + sign * p[n - b]) % MOD;
            }
            k += 1;
        }
        p.push(total);
        if total % MOD == 0 {
            println!("{}", p.len() - 1);
            break;
        }
    }
}
