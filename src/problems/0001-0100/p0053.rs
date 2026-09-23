pub fn run() {
    let mut ans = 0;
    for n in 1..=100 {
        for r in 1..=n {
            let k = (n - r).min(r);
            let mut prod = 1;
            for i in 1..=k {
                prod = prod * (n - k + i) / i;
                if prod > 1_000_000 {
                    ans += 1;
                    break;
                }
            }
        }
    }
    println!("{ans}");
}
