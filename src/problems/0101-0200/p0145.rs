// Created on Oct 08, 2026

pub fn run() {
    let mut ans = 0;

    for d in 1..=9 {
        if d % 2 == 0 {
            ans += 20 * 30u64.pow(d / 2 - 1);
        } else if d % 4 == 3 {
            ans += 100 * 500u64.pow((d - 3) / 4);
        }
    }

    println!("{ans}");
}
