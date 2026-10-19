// Created on Oct 18, 2026

fn solve(n: usize) -> i64 {
    let mut ans = 0;

    let mut two = vec![0; n + 1];
    let mut five = vec![0; n + 1];

    for t in 1..=n {
        let mut x = t;
        while x % 2 == 0 {
            two[t] += 1;
            x /= 2;
        }
        while x % 5 == 0 {
            five[t] += 1;
            x /= 5;
        }
        two[t] += two[t - 1];
        five[t] += five[t - 1];
    }

    for i in 0..=n / 3 {
        for j in i..=(n - i) / 2 {
            let k = n - i - j;

            let two = two[n] - two[i] - two[j] - two[k];
            let five = five[n] - five[i] - five[j] - five[k];

            if two >= 12 && five >= 12 {
                ans += if i == k {
                    1
                } else if i == j || j == k {
                    3
                } else {
                    6
                };
            }
        }
    }

    ans
}

pub fn run() {
    let ans = solve(200_000);
    println!("{ans}");
}
