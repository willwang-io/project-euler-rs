fn solve(n: i64) -> i64 {
    fn dfs(left: usize, prev: usize, memo: &mut [[i64; 10]]) -> i64 {
        if left == 0 {
            return 1;
        }
        if memo[left][prev] != -1 {
            return memo[left][prev];
        }
        let mut ans = 1;
        for digit in 0..=prev {
            ans += dfs(left - 1, digit, memo);
        }
        memo[left][prev] = ans;
        ans
    }

    let mut memo = vec![[-1; 10]; n as usize];
    let mut ans = 0;

    for digit in 1..10 {
        ans += dfs(n as usize - 1, digit, &mut memo) + dfs(n as usize - 1, 9 - digit, &mut memo);
    }

    ans - 9 * n
}

pub fn run() {
    println!("{}", solve(100));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(12951, solve(6));
        assert_eq!(277032, solve(10));
    }
}
