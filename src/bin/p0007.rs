use pe_rs::number_theory::sieve_of_eratosthenes;

fn main() {
    let s = sieve_of_eratosthenes(200000);

    let mut n = 10001;

    for p in 2..200000 {
        if s[p] {
            n -= 1;
        }

        if n == 0 {
            println!("{}", p);
            break;
        }
    }
}
