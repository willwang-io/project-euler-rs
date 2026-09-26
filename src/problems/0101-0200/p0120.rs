use pe_rs::number_theory::mod_pow;

pub fn run() {
    let mut ans = 0;
    for a in 3..=1000u64 {
        let mut cur = 0;
        for n in (1..2000).step_by(2) {
            let tmp = (mod_pow(a - 1, n, a * a) + mod_pow(a + 1, n, a * a)) % (a * a);
            cur = cur.max(tmp);
        }
        ans += cur;
    }
    println!("{ans}");
}
