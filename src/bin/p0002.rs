fn main() {
    let mut a = 1;
    let mut b = 2;

    let mut ans = 0;
    while b < 4_000_000 {
        if b % 2 == 0 {
            ans += b;
        }
        (a, b) = (b, a + b);
    }

    println!("{}", ans);
}
