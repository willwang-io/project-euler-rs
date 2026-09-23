use pe_rs::number_theory::divisors;

pub fn run() {
    let div_sum = |x: u64| -> u64 { divisors(x).iter().sum::<u64>() - x };

    let mut ans = 0;

    for a in 2..10000 {
        let b = div_sum(a);

        if b != a && div_sum(b) == a {
            ans += a;
        }
    }

    println!("{}", ans);
}
