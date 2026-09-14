use pe_rs::number_theory::prime_factors;
use std::collections::HashSet;

fn main() {
    let mut cnt = HashSet::new();
    for a in 2..=100 {
        for b in 2..=100 {
            let fact = prime_factors(a);
            let token = fact
                .iter()
                .map(|&(p, c)| format!("{}-{}", p, c * b))
                .collect::<Vec<_>>()
                .join("#");
            cnt.insert(token);
        }
    }
    println!("{}", cnt.len());
}
