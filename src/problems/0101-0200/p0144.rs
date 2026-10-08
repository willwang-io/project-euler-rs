// Created on Oct 07, 2026

use pe_rs::linear_algebra::vector::Vector;

pub fn run() {
    let a = 4.0f64;
    let b = 1.0;

    let s = Vector::from([0.0, 10.1]);
    let mut p = Vector::from([1.4, -9.6]);

    let mut v = &p - &s;

    let mut cnt = 0;
    loop {
        cnt += 1;
        let n = Vector::from([a * p[0], b * p[1]]);
        let k = 2.0 * (&v * &n) / (&n * &n);
        v = &v - &(&n * k);

        let t = -2.0 * (&n * &v) / (a * v[0] * v[0] + b * v[1] * v[1]);

        p = &p + &(&v * t);
        if p[0].abs() <= 0.01 && p[1] > 0.0 {
            break;
        }
    }
    println!("{cnt}");
}
