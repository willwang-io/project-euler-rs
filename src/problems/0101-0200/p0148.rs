// Created on Oct 09, 2026

fn f(mut n: i64) -> i64 {
    let mut tmp = 1;
    while n > 0 {
        let r = n % 7;
        tmp *= r + 1;
        n /= 7;
    }
    tmp
}

fn solve(n: i64) -> i64 {
    if n == -1 {
        return 0;
    }
    let q = n / 7;
    let r = n % 7;
    28 * solve(q - 1) + f(q) * (r + 1) * (r + 2) / 2
}

pub fn run() {
    let ans = solve(1_000_000_000 - 1);
    println!("{ans}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(2361, solve(100 - 1));
    }
}
