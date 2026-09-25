fn solve() {
    fn dfs(left: usize, prev: usize, cnt: usize, memo: &mut [Vec<Vec<i32>>]) -> i32 {
        if cnt == 0 {
            return 1;
        }
        if memo[left][prev][cnt] != -1 {
            return memo[left][prev][cnt];
        }
        let mut ans = 1;
        for k in prev..62 {
            let score = match k {
                60 => 25,
                61 => 50,
                _ => (k / 20 + 1) * (k % 20 + 1),
            };
            if score <= left {
                ans += dfs(left - score, k, cnt - 1, memo);
            }
        }
        memo[left][prev][cnt] = ans;
        ans
    }

    let mut memo = vec![vec![vec![-1; 3]; 101]; 101];
    let mut ans = 0;
    let limit = 99;

    for j in (1..=20).chain(std::iter::once(25)) {
        ans += dfs(limit - j * 2, 0, 2, &mut memo);
    }

    println!("{ans}");
}

pub fn run() {
    solve();
}
