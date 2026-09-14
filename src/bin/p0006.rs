fn main() {
    let n = 100i64;
    let ans = ((n * (n + 1)) / 2).pow(2) - (n * (n + 1) * (2 * n + 1)) / 6;
    println!("{}", ans);
}
