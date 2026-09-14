use pe_rs::number_theory::prime_factors;

fn main() {
    let ans = prime_factors(600851475143)
        .iter()
        .map(|&(p, _)| p)
        .max()
        .unwrap();
    println!("{}", ans);
}
