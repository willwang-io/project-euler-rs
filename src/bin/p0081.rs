use pe_rs::fetch_input;

fn main() {
    let input = fetch_input("0081_matrix.txt");
    let mat = input
        .unwrap()
        .lines()
        .map(|row| {
            row.split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    const N: usize = 80;
    let mut dp = vec![vec![i64::MAX / 2; N + 1]; N + 1];
    for i in 1..=N {
        for j in 1..=N {
            if i == 1 && i == j {
                dp[i][j] = mat[i - 1][j - 1];
            } else {
                dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]) + mat[i - 1][j - 1];
            }
        }
    }
    println!("{}", dp[N][N]);
}
