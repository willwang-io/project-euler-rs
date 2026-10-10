// Created on Oct 08, 2026

fn count_regular_rectangle(n: usize, m: usize) -> i64 {
    // Same as p85
    (n * (n + 1) * m * (m + 1) / 4) as i64
}

fn count_aztec_rectangle(mut n: usize, mut m: usize) -> i64 {
    if n > m {
        (n, m) = (m, n);
    }

    if n == 1 {
        return (m - 1) as i64;
    }

    let sz = n + m - 2;
    let mut grid = vec![];

    for r in 0..sz {
        let start = (n - 2).saturating_sub(r).max(r.saturating_sub(n - 1));
        let w = if r < n - 1 {
            2 * (r + 1)
        } else if r < m - 1 {
            2 * n - 1
        } else {
            2 * (sz - r)
        };
        grid.push((start, start + w));
    }

    let mut cnt = 0;

    for i in 0..sz {
        let mut left = 0;
        let mut right = sz;

        for &(start, end) in &grid[i..] {
            left = left.max(start);
            right = right.min(end);
            let w = right.saturating_sub(left) as i64;

            cnt += w * (w + 1) / 2;
        }
    }

    cnt
}

fn count_all_rectangles(n: usize, m: usize) -> i64 {
    let mut ans = 0;

    // It can be reduced further due to symmetry. But this is already fast enough, and I am lazy...
    for i in 1..=n {
        for j in 1..=m {
            ans += count_regular_rectangle(i, j) + count_aztec_rectangle(i, j);
        }
    }

    ans
}

pub fn run() {
    let ans = count_all_rectangles(47, 43);
    println!("{ans}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(72, count_all_rectangles(3, 2));
    }
}
