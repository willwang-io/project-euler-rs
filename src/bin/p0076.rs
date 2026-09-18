fn main() {
    let n = 100;
    let mut dp = vec![0i64; n + 1];
    dp[0] = 1;

    for i in 1..n {
        for j in i..=n {
            dp[j] += dp[j - i];
        }
    }
    println!("{}", dp[n]);
}
