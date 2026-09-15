use pe_rs::utility::next_permutation;
use std::collections::HashSet;

fn main() {
    let helper = |x: &mut [i32]| -> i32 {
        let f = |y: &[i32]| -> i32 { y.iter().fold(0, |acc, &x| acc * 10 + x) };

        for i in 1..10 {
            for j in i + 1..9 {
                let a = f(&x[..i]);
                let b = f(&x[i..j]);
                let c = f(&x[j..]);
                if a * b == c {
                    return c;
                }
            }
        }
        0
    };

    let mut a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut cnt = HashSet::new();

    loop {
        cnt.insert(helper(&mut a));
        if !next_permutation(&mut a) {
            break;
        }
    }

    println!("{}", cnt.iter().sum::<i32>());
}
