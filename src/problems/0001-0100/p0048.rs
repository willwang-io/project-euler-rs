const MOD: i64 = 10000000000;

pub fn run() {
    let mut ans = 0i64;
    for i in 1..=1000 {
        let mut cur = 1;
        for _ in 0..i {
            cur = (cur * i) % MOD;
        }
        ans = (ans + cur) % MOD;
    }
    println!("{ans}");
}
