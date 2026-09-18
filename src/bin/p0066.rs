use pe_rs::big_int::from_digits_le;
use pe_rs::number_theory::continued_fractions::{convergent, period};

fn helper(n: i32) -> i128 {
    let mut tmp = period(n);
    if (tmp.len() - 1) % 2 == 0 {
        tmp.pop();
    } else {
        tmp.extend_from_within(1..tmp.len() - 1);
    }
    let (a, _) = convergent(0, tmp.len(), &tmp);
    from_digits_le(&a)
}

fn main() {
    let mut ans = 0;
    let mut mx = 0;
    for d in 2..=1000i32 {
        if d.isqrt() * d.isqrt() == d {
            continue;
        }
        let tmp = helper(d);
        if tmp > mx {
            mx = tmp;
            ans = d;
        }
    }
    println!("{ans}");
}
