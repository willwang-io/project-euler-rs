fn main() {
    let coins = [1, 2, 5, 10, 20, 50, 100, 200];
    let mut dp = vec![0i64; 201];
    dp[0] = 1;

    for c in coins {
        for i in c..=200 {
            dp[i] += dp[i - c];
        }
    }
    println!("{}", dp[200]);
}
