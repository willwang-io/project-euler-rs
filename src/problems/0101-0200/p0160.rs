// Created on Nov 07, 2026

use pe_rs::number_theory::{gcd, legendre, mod_pow};

const M: i64 = 100_000;

fn solve(m: i64) -> i64 {
    let mut prefix = vec![1; M as usize];
    let mut block_prod = 1;

    for k in 1..M {
        if gcd(k, 10) == 1 {
            block_prod = block_prod * k % M;
        }
        prefix[k as usize] = block_prod;
    }

    let mut prod = 1;
    for a in 0.. {
        let p2 = 2i64.pow(a);
        if p2 >= m {
            break;
        }
        for b in 0.. {
            let tmp = p2 * 5i64.pow(b);
            if tmp > m {
                break;
            }
            let cur_prod = prefix[((m / tmp) % M) as usize];
            prod = prod * cur_prod % M;
        }
    }

    let e2 = legendre(m, 2);
    let e5 = legendre(m, 5);

    mod_pow(2, e2 - e5, M) * prod % M
}

pub fn run() {
    println!("{}", solve(1_000_000_000_000));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(36288, solve(9));
        assert_eq!(36288, solve(10));
        assert_eq!(17664, solve(20));
    }
}
