use pe_rs::number_theory::sieve_of_eratosthenes;

pub fn run() {
    let s = sieve_of_eratosthenes(1_000_000);
    let mut n = 0;
    let mut i = 11;
    let mut ans = 0;

    while n < 11 {
        if s[i] {
            let mut j = i;
            let mut ok = true;
            while j > 0 {
                ok &= s[j];
                j /= 10;
            }
            let mut k = 10;
            while k < i {
                ok &= s[i % k];
                k *= 10;
            }
            if ok {
                ans += i;
                n += 1;
            }
        }
        i += 1;
    }

    println!("{ans}");
}
