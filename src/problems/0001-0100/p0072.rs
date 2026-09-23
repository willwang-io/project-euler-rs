use pe_rs::number_theory::euler_totient;

pub fn run() {
    let mut x = 0;
    for i in 2..=1_000_000 {
        x += euler_totient(i);
    }
    println!("{x}");
}
