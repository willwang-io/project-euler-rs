// Created on Oct 02, 2026

pub fn run() {
    let lmt = 1_000_000;
    let mut cnt = vec![0u32; lmt];
    for a in 1.. {
        if a * a >= lmt {
            break;
        }
        for b in (a + 2 * (a % 2)..=(lmt - 1) / a).step_by(4) {
            cnt[a * b] += 1;
            if a != b && b < 3 * a {
                cnt[a * b] += 1;
            }
        }
    }

    let ans = cnt.iter().filter(|&&c| c == 10).count();
    println!("{ans}");
}
