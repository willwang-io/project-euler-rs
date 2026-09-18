use pe_rs::number_theory::continued_fractions::period;

fn main() {
    let mut ans = 0;
    for n in 0..=10_000 {
        if (period(n).len() - 1) % 2 == 1 {
            ans += 1;
        }
    }
    println!("{ans}");
}
