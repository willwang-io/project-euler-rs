use pe_rs::number_theory::{is_prime, sieve_of_eratosthenes};

fn main() {
    let s = sieve_of_eratosthenes(1000);
    let p = (0..1000_i64).filter(|&i| s[i as usize]).collect::<Vec<_>>();

    let mut longest = 0;
    let mut ans = 0;
    for b in p {
        for a in -999..1000 {
            let mut n = 0;
            while is_prime((n * n + a * n + b) as u64) {
                n += 1;
            }
            if n > longest {
                longest = n;
                ans = a * b;
            }
        }
    }

    println!("{}", ans);
}
