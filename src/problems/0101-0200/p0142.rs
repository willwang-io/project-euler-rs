// Created on Oct 06, 2026

fn is_square(n: i64) -> bool {
    n.isqrt() * n.isqrt() == n
}

pub fn run() {
    let lmt = 1000;

    for b in 1..lmt {
        let b2 = b * b;
        for c in b + 1..lmt {
            let c2 = c * c;
            if !is_square(c2 - b2) {
                continue;
            }
            for a in c + 1..lmt {
                let a2 = a * a;
                let d2 = a2 + b2 - c2;
                if (a2 + b2) % 2 != 0 || 2 * c2 <= a2 + b2 {
                    continue;
                }
                if !is_square(a2 - c2) || !is_square(d2) {
                    continue;
                }
                println!("{}", (a * a - b * b + 2 * c * c) / 2);
            }
        }
    }
}
