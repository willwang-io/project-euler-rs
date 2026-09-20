use pe_rs::number_theory::continued_fractions::period;

fn helper(n: usize) -> u128 {
    let cf = period(n);
    let len = cf.period_len();
    let index = if len % 2 == 0 { len - 1 } else { 2 * len - 1 };
    let (a, _) = cf.convergent(index);
    a.to_u128()
}

fn main() {
    let mut ans = 0;
    let mut mx = 0;
    for d in 2..=1000usize {
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
