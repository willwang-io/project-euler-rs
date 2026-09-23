use pe_rs::big_int::BigInt;

fn main() {
    let mut ans = 0;
    for a in 1..100 {
        let bytes_a = BigInt::from(a);
        let mut cur = BigInt::from(a);
        for _ in 1..100 {
            cur = &cur * &bytes_a;
            ans = ans.max(cur.digits_sum());
        }
    }
    println!("{ans}");
}
