use pe_rs::number_theory::continued_fraction_period;

fn main() {
    let mut ans = 0;
    for n in 0..=10_000 {
        if (continued_fraction_period(n).len() - 1) % 2 == 1 {
            ans += 1;
        }
    }
    println!("{ans}");
}
