use pe_rs::number_theory::gcd;

fn main() {
    let mut cnt = vec![0; 1001];

    for m in 2..20 {
        for n in 1..m {
            if gcd(m, n) != 1 || (m - n) % 2 == 0 {
                continue;
            }
            let a = m * m - n * n;
            let b = 2 * m * n;
            let c = m * m + n * n;
            let mut i = 1;
            loop {
                let cur = i * (a + b + c);
                if cur > 1000 {
                    break;
                }
                cnt[cur as usize] += 1;
                i += 1;
            }
        }
    }

    let mx = *cnt.iter().max().unwrap();
    let ans = cnt.iter().position(|&x| x == mx).unwrap();
    println!("{}", ans);
}
