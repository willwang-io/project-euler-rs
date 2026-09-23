fn sakamoto_method(mut y: usize, m: usize, d: usize) -> usize {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    if m < 3 {
        y -= 1;
    }
    (y + y / 4 - y / 100 + y / 400 + t[m - 1] + d) % 7
}

pub fn run() {
    let mut ans = 0;

    for y in 1901..=2000 {
        for m in 1..=12 {
            if sakamoto_method(y, m, 1) == 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
