use pe_rs::number_theory::sieve_of_eratosthenes;

pub fn run() {
    let s = sieve_of_eratosthenes(1_000_000);
    let mut ans = 0;

    for mut i in 2..1_000_000 {
        let n = i.to_string().len();
        let mut ok = true;
        for _ in 0..n {
            if !s[i] {
                ok = false;
                break;
            }
            i = i % 10 * 10usize.pow(n as u32 - 1) + i / 10;
        }
        if ok {
            ans += 1;
        }
    }

    println!("{ans}");
}
