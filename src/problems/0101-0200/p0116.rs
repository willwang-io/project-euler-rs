fn f(m: usize, n: usize) -> i64 {
    fn dfs(n: usize, m: usize, memo: &mut [i64]) -> i64 {
        if n == 0 {
            return 1;
        }
        if memo[n] != -1 {
            return memo[n];
        }
        let mut ans = dfs(n - 1, m, memo);
        if n >= m {
            ans += dfs(n - m, m, memo);
        }
        memo[n] = ans;
        ans
    }
    let mut memo = vec![-1; n + 1];
    dfs(n, m, &mut memo)
}

fn solve(n: usize) -> i64 {
    f(2, n) + f(3, n) + f(4, n) - 3
}

pub fn run() {
    println!("{}", solve(50));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(12, solve(5));
    }
}
