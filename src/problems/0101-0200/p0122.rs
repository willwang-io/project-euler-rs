// Created on Sep 26, 2026

fn solve(n: usize) -> usize {
    fn dfs(i: usize, n: usize, path: &mut Vec<usize>, shortest: &mut [usize]) {
        for j in (0..path.len()).rev() {
            let next = i + path[j];
            if next <= n && path.len() <= shortest[next] {
                shortest[next] = path.len();
                path.push(next);
                dfs(next, n, path, shortest);
                path.pop();
            }
        }
    }

    let mut shortest = vec![usize::MAX; n + 1];
    shortest[1] = 0;
    dfs(1, n, &mut vec![1], &mut shortest);
    shortest[1..=n].iter().sum()
}

pub fn run() {
    let ans = solve(200);
    println!("{ans}");
}
