use pe_rs::number_theory::prime_factors;

fn main() {
    let mut i = 1;
    loop {
        if prime_factors(i).len() == 4
            && prime_factors(i + 1).len() == 4
            && prime_factors(i + 2).len() == 4
            && prime_factors(i + 3).len() == 4
        {
            println!("{i}");
            break;
        }
        i += 1;
    }
}
