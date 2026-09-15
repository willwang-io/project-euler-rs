use pe_rs::utility::next_permutation;

fn main() {
    let ok = |x: &[i64]| -> bool {
        let p = vec![0, 0, 1, 2, 3, 5, 7, 11, 13, 17];
        let mut cur = 0;
        for (i, (&a, b)) in x.iter().zip(p).enumerate() {
            cur = (cur * 10 + a) % 1000;
            if i > 1 && cur % b != 0 {
                return false;
            }
        }
        true
    };

    let mut a: Vec<i64> = (0..10).collect();
    let mut ans = 0;

    loop {
        if ok(&a) {
            ans += a.iter().fold(0, |acc, &x| acc * 10 + x);
        }
        if !next_permutation(&mut a) {
            break;
        }
    }

    println!("{ans}");
}
