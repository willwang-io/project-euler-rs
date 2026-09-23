use pe_rs::number_theory::prime_factors;

pub fn run() {
    let ans = prime_factors(600851475143)
        .iter()
        .map(|&(p, _)| p)
        .max()
        .unwrap();
    println!("{}", ans);
}
