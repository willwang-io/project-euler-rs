use pe_rs::number_theory::euler_totient;

pub fn run() {
    let mut mx_n = 0;
    let mut mx_phi = 1;
    let mut ans = 0;

    for n in 2..=1_000_000 {
        let phi = euler_totient(n);
        if n * mx_phi > mx_n * phi {
            mx_n = n;
            mx_phi = phi;
            ans = n;
        }
    }
    println!("{ans}");
}
