use crate::big_int::{add, mul, to_digit_le};
use std::collections::HashSet;

pub fn period(n: i32) -> Vec<i32> {
    let mut p = 0;
    let mut q = 1;
    let mut d = vec![];
    let mut seen = HashSet::new();

    loop {
        let a = (((n as f64).sqrt() + p as f64) / q as f64).floor() as i32;
        if !seen.insert((p, q)) || q == 0 {
            break;
        }
        d.push(a);
        p = a * q - p;
        q = (n - p * p) / q;
    }

    d
}

pub fn convergent(i: usize, n: usize, arr: &[i32]) -> (Vec<u8>, Vec<u8>) {
    if i + 1 == n {
        return (to_digit_le(arr[i]), vec![1]);
    }
    let (num, den) = convergent(i + 1, n, arr);
    let new_num = add(&mul(&to_digit_le(arr[i]), &num), &den);
    (new_num, num)
}
