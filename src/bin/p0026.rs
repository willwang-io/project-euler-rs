use pe_rs::number_theory::{divisors, euler_totient, mod_pow};

fn main() {
    let multiplicity = |p: u64, mut n: u64| -> u32 {
        let mut cnt = 0;
        while n % p == 0 {
            n /= p;
            cnt += 1;
        }
        cnt
    };

    let mut longest = 0u64;
    let mut ans = 0;

    for d in 2..1000 {
        let x = 2u64.pow(multiplicity(2, d));
        let y = 5u64.pow(multiplicity(5, d));
        let n = d / x / y;

        let phi_n = euler_totient(n);
        let div = divisors(phi_n);
        for m in div {
            if mod_pow(10, m, n) == 1 {
                if longest < m {
                    longest = m;
                    ans = d;
                }
                break;
            }
        }
    }

    println!("{}", ans);
}
