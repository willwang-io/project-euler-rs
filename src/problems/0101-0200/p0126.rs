// Created on Sep 27, 2026

// For a cuboid with dimension (a, b, c), we can brute-force the number of cubes needed in the
// first few layers, and apply polynomial interpolation. This suggests that all these polynomial
// follow the pattern 4x^2 + Px + Q, where P = 4 * (a + b + c - 1) and Q = 2 * (ab + ac + bc), with
// x = 0 representing the first layer.

fn helper(limit: usize) -> Vec<u32> {
    let mut cnt = vec![0; limit + 1];

    for a in 1.. {
        if 6 * a * a > limit {
            break;
        }

        for b in a.. {
            if 2 * (2 * a * b + b * b) > limit {
                break;
            }

            for c in b.. {
                let mut cur = 2 * (a * b + a * c + b * c);
                if cur > limit {
                    break;
                }
                let mut step = 4 * (a + b + c);
                while cur <= limit {
                    cnt[cur] += 1;
                    cur += step;
                    step += 8;
                }
            }
        }
    }

    cnt
}

pub fn run() {
    let mut limit = 1_000;

    loop {
        let cnt = helper(limit);

        if let Some(ans) = cnt.iter().position(|&v| v == 1_000) {
            println!("{ans}");
            break;
        }

        limit *= 2;
    }
}
