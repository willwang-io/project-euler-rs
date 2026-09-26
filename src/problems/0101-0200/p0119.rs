use pe_rs::big_int::BigInt;

pub fn run() {
    let mut ans = vec![];
    for i in 2..100 {
        let mut x = BigInt::from(i);
        for _ in 1..20 {
            x = x * BigInt::from(i);
            if x.digits_sum() == i {
                ans.push(x.clone());
            }
        }
    }
    ans.sort_unstable();
    println!("{}", ans[29]);
}
