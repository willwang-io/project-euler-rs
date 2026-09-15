fn main() {
    let is_tri = |x: i64| -> bool {
        let tmp = 1 + 8 * x;
        let tmp_sqrt = tmp.isqrt();
        if tmp_sqrt * tmp_sqrt != tmp {
            return false;
        }
        (tmp_sqrt - 1) % 2 == 0
    };

    let is_pent = |x: i64| -> bool {
        let tmp = 1 + 24 * x;
        let tmp_sqrt = tmp.isqrt();
        if tmp_sqrt * tmp_sqrt != tmp {
            return false;
        }
        (1 + tmp_sqrt) % 6 == 0
    };

    for i in 1..200000 {
        let h = i * (2 * i - 1);
        if is_pent(h) && is_tri(h) {
            println!("{}", h);
        }
    }
}
