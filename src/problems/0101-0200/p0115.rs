fn f(m: usize, n: usize) -> i64 {
    fn dfs(n: usize, m: usize, memo: &mut [i64]) -> i64 {
        if n == 0 {
            return 1;
        }
        if memo[n] != -1 {
            return memo[n];
        }
        let mut ans = dfs(n - 1, m, memo);
        for i in m..=n {
            ans += if n == i { 1 } else { dfs(n - i - 1, m, memo) };
        }
        memo[n] = ans;
        ans
    }
    let mut memo = vec![-1; n + 1];
    dfs(n, m, &mut memo)
}

pub fn run() {
    for n in 1.. {
        if f(50, n) > 1_000_000 {
            println!("{n}");
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(880711, f(10, 56));
        assert_eq!(1148904, f(10, 57));
    }
}
