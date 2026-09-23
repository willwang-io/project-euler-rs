pub fn run() {
    let p = 3;
    let q = 7;
    const N: i32 = 1_000_000;

    let mut b = 0;
    for i in (1..N).rev() {
        if (i * p) % q == 1 {
            b = i;
            break;
        }
    }
    let a = (p * b - 1) / q;
    println!("{a}");
}
