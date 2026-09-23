use pe_rs::number_theory::euler_totient;

fn ok(mut n: u64) -> bool {
    let mut phi_n = euler_totient(n);
    let mut cnt = [0; 10];
    while n > 0 {
        cnt[(n % 10) as usize] += 1;
        n /= 10;
    }
    while phi_n > 0 {
        cnt[(phi_n % 10) as usize] -= 1;
        phi_n /= 10;
    }
    cnt.iter().all(|&x| x == 0)
}
pub fn run() {
    let mut mx_n = u64::MAX >> 32;
    let mut mx_phi = 1;
    let mut ans = 0;

    for n in 2..10_000_000 {
        let phi = euler_totient(n);
        if n * mx_phi < mx_n * phi && ok(n) {
            mx_n = n;
            mx_phi = phi;
            ans = n;
        }
    }
    println!("{ans}");
}
