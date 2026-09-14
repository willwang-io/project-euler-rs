use pe_rs::number_theory::prime_factors;

fn main() {
    let mut i = 1;
    loop {
        let n = i * (i + 1) / 2;
        let factors = prime_factors(n);
        let d: u32 = factors.iter().map(|&(_, c)| c + 1).product();
        if d > 500 {
            println!("{}", n);
            break;
        }
        i += 1;
    }
}
