use pe_rs::big_int::BigInt;

pub fn run() {
    let mut a = BigInt::from(1);
    let mut b = BigInt::from(2);
    let mut ans = 0;

    for _ in 1..1000 {
        let tmp = a + &b + &b;
        a = b;
        b = tmp;
        if (&a + &b).digit_count() > b.digit_count() {
            ans += 1;
        }
    }

    println!("{ans}");
}
