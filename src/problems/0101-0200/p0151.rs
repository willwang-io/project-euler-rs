// Created on Oct 12, 2026

fn dfs(batch: usize, cnt: &mut [i32]) -> f64 {
    if batch == 16 {
        return 0.0;
    }
    let total: i32 = cnt.iter().sum();
    let mut ans = 0.0;
    if total == 1 && batch > 1 {
        ans += 1.0;
    }
    for i in 1..cnt.len() {
        if cnt[i] > 0 {
            let prob = f64::from(cnt[i]) / f64::from(total);
            cnt[i] -= 1;
            for j in i + 1..cnt.len() {
                cnt[j] += 1;
            }
            ans += prob * dfs(batch + 1, cnt);
            cnt[i] += 1;
            for j in i + 1..cnt.len() {
                cnt[j] -= 1;
            }
        }
    }
    ans
}

pub fn run() {
    let mut paper = [0, 1, 0, 0, 0, 0];
    let ans = dfs(1, &mut paper);

    println!("{:.6}", ans);
}
