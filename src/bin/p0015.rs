fn main() {
    const N: usize = 20;

    let mut dp = vec![vec![0i64; N + 2]; N + 2];

    dp[0][1] = 1;
    dp[1][0] = 1;

    for i in 1..=N + 1 {
        for j in 1..=N + 1 {
            dp[i][j] += dp[i - 1][j] + dp[i][j - 1];
        }
    }

    println!("{}", dp[N + 1][N + 1] / 2);
}
