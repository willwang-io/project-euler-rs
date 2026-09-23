pub fn run() {
    const LIMIT: i64 = 1_000_000_000;
    let mut ans = 0;

    for a in 5..=(LIMIT + 1) / 3 {
        for d in [-1, 1] {
            let x = (a - d) * (3 * a + d);
            if x.isqrt() * x.isqrt() == x {
                ans += 3 * a + d;
            }
        }
    }

    println!("{ans}");
}
