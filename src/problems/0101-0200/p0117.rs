fn f(n: usize) -> i64 {
    fn dfs(n: usize, memo: &mut [i64]) -> i64 {
        if n == 0 {
            return 1;
        }
        if memo[n] != -1 {
            return memo[n];
        }
        let mut ans = dfs(n - 1, memo);
        for m in 2..=4 {
            if n >= m {
                ans += dfs(n - m, memo);
            }
        }
        memo[n] = ans;
        ans
    }
    let mut memo = vec![-1; n + 1];
    dfs(n, &mut memo)
}

pub fn run() {
    println!("{}", f(50));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(15, f(5));
    }
}
