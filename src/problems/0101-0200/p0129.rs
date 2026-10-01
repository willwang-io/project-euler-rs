// Created on Sep 30, 2026

use pe_rs::number_theory::multiplicative_order;

pub fn run() {
    for n in 1_000_001.. {
        if n % 2 != 0 && n % 5 != 0 && multiplicative_order(10, 9 * n) > 1_000_000 {
            println!("{n}");
            break;
        }
    }
}
