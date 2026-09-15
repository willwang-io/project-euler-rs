fn main() {
    let is_pent = |x: i64| -> bool {
        let tmp = 1 + 24 * x;
        if tmp.isqrt() * tmp.isqrt() != tmp {
            return false;
        }
        (1 + tmp.isqrt()) % 6 == 0
    };

    let mut ans = i64::MAX;
    for x in 1..3000 {
        for y in 1..x {
            let p1: i64 = x * (3 * x - 1) / 2;
            let p2: i64 = y * (3 * y - 1) / 2;

            if is_pent(p1 - p2) && is_pent(p1 + p2) {
                ans = ans.min(p1 - p2);
            }
        }
    }
    println!("{ans}");
}
