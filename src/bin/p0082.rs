use pe_rs::fetch_input;

fn main() {
    let input = fetch_input("0082_matrix.txt");
    let mat = input
        .unwrap()
        .lines()
        .map(|row| {
            row.split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let n = mat.len();
    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        dp[i][0] = mat[i][0];
    }

    for j in 1..n {
        for i in 0..n {
            dp[i][j] = dp[i][j - 1] + mat[i][j];
        }
        for k in 1..n {
            dp[k][j] = dp[k][j].min(dp[k - 1][j] + mat[k][j]);
        }
        for k in (0..n - 1).rev() {
            dp[k][j] = dp[k][j].min(dp[k + 1][j] + mat[k][j]);
        }
    }

    let mut ans = i64::MAX / 4;
    for i in 0..n {
        ans = ans.min(dp[i][n - 1]);
    }
    println!("{ans}");
}
