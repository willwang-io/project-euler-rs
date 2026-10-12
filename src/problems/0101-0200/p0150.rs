// Created on Oct 10, 2026

fn gen_triangle_grid() -> Vec<Vec<i64>> {
    let n = 1000;
    let m = 1000 * (1000 + 1) / 2;
    let mut a: Vec<i64> = Vec::with_capacity(m);
    let mut t = 0;

    for _ in 0..m {
        t = (615949 * t + 797807) % (1 << 20);
        a.push(t - (1 << 19));
    }

    assert_eq!(273519, a[0]);
    assert_eq!(-153582, a[1]);
    assert_eq!(450905, a[2]);

    let mut iter = a.into_iter();
    let mut ans = Vec::with_capacity(n);

    for i in 1..=n {
        let tmp: Vec<_> = iter.by_ref().take(i).collect();
        ans.push(tmp);
    }
    ans
}

fn solve(grid: &[Vec<i64>]) -> i64 {
    let n = grid.len();
    let mut suf_sum_arr = Vec::with_capacity(grid.len());

    for row in grid {
        let mut cur = vec![0i64; row.len() + 1];
        for j in (0..row.len()).rev() {
            cur[j] = cur[j + 1] + row[j];
        }
        suf_sum_arr.push(cur);
    }

    let mut ans = i64::MAX;

    for (i, row) in suf_sum_arr.iter().enumerate() {
        for j in 0..row.len() - 1 {
            let mut cur = 0;
            for k in i..n {
                cur += suf_sum_arr[k][j] - suf_sum_arr[k][j + k - i + 1];
                ans = ans.min(cur);
            }
        }
    }

    ans
}

pub fn run() {
    let grid = gen_triangle_grid();
    let ans = solve(&grid);

    println!("{ans}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let grid = vec![
            vec![15],
            vec![-14, -7],
            vec![20, -13, -5],
            vec![-3, 8, 23, -26],
            vec![1, -4, -5, -18, 5],
            vec![-16, 31, 2, 9, 28, 3],
        ];

        assert_eq!(-42, solve(&grid));
    }
}
