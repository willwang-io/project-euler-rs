// Created on Nov 02, 2026

use std::collections::HashMap;

fn solve(n: i32) -> i64 {
    fn dfs(
        n: i32,
        cnt: i32,
        smaller: i32,
        used: i32,
        memo: &mut HashMap<(i32, i32, i32), i64>,
    ) -> i64 {
        if cnt > 1 {
            return 0;
        }

        if used == n {
            return if cnt == 1 { 1 } else { 0 };
        }

        if let Some(&val) = memo.get(&(cnt, smaller, used)) {
            return val;
        }

        let mut ans = 0;

        for c in 0..26 - used {
            ans += dfs(n, cnt + if c >= smaller { 1 } else { 0 }, c, used + 1, memo);
        }

        memo.insert((cnt, smaller, used), ans);
        ans
    }

    let mut memo = HashMap::new();
    dfs(n, 0, 26, 0, &mut memo)
}

pub fn run() {
    let ans = (0..=26).map(solve).max().unwrap();
    println!("{ans}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(10400, solve(3));
    }
}
