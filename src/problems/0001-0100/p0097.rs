use pe_rs::number_theory::mod_pow;

pub fn run() {
    println!(
        "{}",
        (28433 * mod_pow(2, 7830457, 10_000_000_000) + 1) % 10_000_000_000
    );
}
