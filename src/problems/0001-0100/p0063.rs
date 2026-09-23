pub fn run() {
    let mut ans = 0;
    for a in 1..50 {
        for b in 1..50 {
            let d = ((b as f64) * (a as f64).log10()).floor() as u64 + 1;
            if d == b {
                ans += 1;
            }
        }
    }
    println!("{ans}");
}
