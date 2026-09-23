pub fn run() {
    let mut b1 = 1u64;
    let mut b2 = 3u64;
    let mut r1 = 0u64;
    let mut r2 = 1u64;

    while b2 + r2 <= 1e12 as u64 {
        (b1, b2) = (b2, 6 * b2 - b1 - 2);
        (r1, r2) = (r2, 6 * r2 - r1);
    }

    println!("{b2}");
}
