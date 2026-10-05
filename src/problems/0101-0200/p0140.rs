// Created on Oct 04, 2026

fn f(p: i64, q: i64) -> i64 {
    let a = 3 * p * p + p * q;
    let b = q * q - p * q - p * p;
    a / b
}

pub fn run() {
    let mut golden_nuggets = vec![];

    let yield_golden_nuggets = |mut p: i64, mut q: i64, arr: &mut Vec<i64>| {
        for _ in 0..20 {
            arr.push(f(p, q));
            (p, q) = (p + q, p + 2 * q);
        }
    };

    yield_golden_nuggets(1, 2, &mut golden_nuggets);
    yield_golden_nuggets(2, 5, &mut golden_nuggets);

    golden_nuggets.sort_unstable();

    let ans: i64 = golden_nuggets.iter().take(30).sum();
    println!("{ans}");
}
