// Created on Nov 01, 2026

fn f(n: i64, d: i64) -> i64 {
    let mut ans = 0;
    let mut cur = 1;

    while cur <= n {
        let x = cur * 10;
        ans += n / x * cur;
        ans += (n % x - d * cur + 1).clamp(0, cur);
        cur *= 10;
    }

    ans
}

fn s(d: i64) -> i64 {
    fn search(d: i64, lo: i64, hi: i64) -> i64 {
        if f(lo, d) > hi || f(hi, d) < lo {
            return 0;
        }
        if lo == hi {
            return if f(lo, d) == lo { lo } else { 0 };
        }
        let mid = lo.midpoint(hi);
        search(d, lo, mid) + search(d, mid + 1, hi)
    }

    search(d, 0, 1_000_000_000_000_000)
}

pub fn run() {
    let ans = (1..10).fold(0, |acc, d| acc + s(d));
    println!("{ans}");
}

#[cfg(test)]
mod test {
    use super::*;

    fn brute_force_f(n: i64, d: i64) -> i64 {
        let mut ans = 0;
        for i in 0..=n {
            let mut j = i;
            while j > 0 {
                if j % 10 == d {
                    ans += 1;
                }
                j /= 10;
            }
        }
        ans
    }

    #[test]
    fn test_f() {
        assert_eq!(f(199981, 1), 199981);

        for d in 1..10 {
            for n in 0..1000 {
                assert_eq!(brute_force_f(n, d), f(n, d));
            }
        }
    }

    #[test]
    fn example() {
        assert_eq!(22786974071, s(1))
    }
}
