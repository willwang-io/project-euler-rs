fn main() {
    let n = 1001;
    let mut ans = 1;

    for i in 0..n / 2 {
        let a = 4 * i * i + 6 * i + 3;
        let b = 4 * i * i + 8 * i + 5;
        let c = 4 * (i + 1) * (i + 1) + 2 * (i + 1) + 1;
        let d = 4 * (i + 1) * (i + 1) + 4 * (i + 1) + 1;
        ans += a + b + c + d;
    }

    println!("{ans}");
}
