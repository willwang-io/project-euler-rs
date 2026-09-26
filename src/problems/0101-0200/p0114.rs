fn solve(n: usize) -> i64 {
    fn dfs(n: usize, memo: &mut [i64]) -> i64 {
        if n == 0 {
            return 1;
        }
        if memo[n] != -1 {
            return memo[n];
        }
        let mut ans = dfs(n - 1, memo);
        for i in 3..=n {
            ans += if n == i { 1 } else { dfs(n - i - 1, memo) };
        }
        memo[n] = ans;
        ans
    }
    let mut memo = vec![-1; n + 1];
    dfs(n, &mut memo)
}

pub fn run() {
    println!("{}", solve(50));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(17, solve(7));
    }
}
