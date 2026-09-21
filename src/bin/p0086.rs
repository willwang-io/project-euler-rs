fn main() {
    let mut total = 0;
    for m in 1_i64.. {
        for x in 2..=2 * m {
            let tmp = x * x + m * m;
            if tmp.isqrt() * tmp.isqrt() == tmp {
                let mn = (x - m).max(1);
                let mx = x / 2;
                total += mx - mn + 1;
            }
        }
        if total > 1_000_000 {
            println!("{m}");
            break;
        }
    }
}
