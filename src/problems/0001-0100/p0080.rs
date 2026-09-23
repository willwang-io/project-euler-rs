use pe_rs::big_int::BigInt;
use pe_rs::number_theory::continued_fractions::period;

fn square_root_decimal_digits_sum(n: usize) -> BigInt {
    let cf = period(n);
    let (mut n, d) = cf.convergent(200);
    let mut sum = BigInt::from(0);
    for _ in 0..100 {
        let (x, y) = n.div_rem(&d);
        if y == BigInt::from(0) {
            break;
        }
        sum = sum + &x;
        n = y * &BigInt::from(10);
    }
    sum
}

pub fn run() {
    let mut ans = BigInt::from(0);
    for i in 1..=100 {
        ans = ans + square_root_decimal_digits_sum(i);
    }
    println!("{ans}");
}
