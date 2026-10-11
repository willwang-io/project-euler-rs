// Created on Oct 10, 2026

fn gen_grid() -> Vec<Vec<i64>> {
    let n = 2000;
    let mut s = Vec::with_capacity(n * n);

    for k in 1..=n * n {
        let x = if k <= 55 {
            let k = k as i64;
            (100_003 - 200_003 * k + 300_007 * k.pow(3)) % 1_000_000 - 500_000
        } else {
            (s[k - 25] + s[k - 56] + 1_000_000) % 1_000_000 - 500_000
        };
        s.push(x);
    }

    assert_eq!(-393027, s[9]);
    assert_eq!(86613, s[99]);

    s.chunks(n).map(|row| row.to_vec()).collect()
}

fn solve(grid: &[Vec<i64>]) -> i64 {
    let n = grid.len();

    let kadane = |x: usize, y: usize, dx: i32, dy: i32| -> i64 {
        let mut x = x as i32;
        let mut y = y as i32;
        let n = n as i32;

        let mut mx = i64::MIN;
        let mut cur = 0;

        while x >= 0 && y >= 0 && x < n && y < n {
            let val = grid[x as usize][y as usize];
            cur = val.max(cur + val);
            mx = mx.max(cur);
            x += dx;
            y += dy;
        }

        mx
    };

    let mut ans = 0;

    for i in 0..n {
        ans = ans
            .max(kadane(i, 0, 0, 1))
            .max(kadane(0, i, 1, 0))
            .max(kadane(0, i, 1, 1))
            .max(kadane(i, 0, 1, 1))
            .max(kadane(0, n - i - 1, 1, -1))
            .max(kadane(n - i - 1, n - 1, 1, -1))
    }

    ans
}

pub fn run() {
    let grid = gen_grid();
    let ans = solve(&grid);
    
    println!("{ans}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let grid = vec![
            vec![-2, 5, 3, 2],
            vec![9, -6, 5, 1],
            vec![3, 2, 7, 3],
            vec![-1, 8, -4, 8],
        ];
        assert_eq!(16, solve(&grid));
    }
}
