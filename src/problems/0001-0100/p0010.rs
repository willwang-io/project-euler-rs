use pe_rs::number_theory::sieve_of_eratosthenes;

pub fn run() {
    let s = sieve_of_eratosthenes(2_000_000);
    let ans = 2
        + (3..2_000_000)
            .step_by(2)
            .fold(0, |acc, x| acc + if s[x] { x } else { 0 });
    println!("{}", ans);
}
