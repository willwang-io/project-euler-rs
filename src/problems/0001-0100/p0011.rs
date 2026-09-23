use pe_rs::fetch_input;

pub fn run() {
    let input = fetch_input("0011.txt").unwrap();
    let grid = input
        .split('\n')
        .map(|row| {
            row.split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut ans = 0;

    for row in &grid {
        for group in row.windows(4) {
            ans = ans.max(group.iter().product());
        }
    }

    for rows in grid.windows(4) {
        let w = rows[0].len();

        for col in 0..w {
            let v = (0..4).map(|i| rows[i][col]).product();
            ans = ans.max(v);
        }

        for col in 0..w.saturating_sub(3) {
            let dr = (0..4).map(|i| rows[i][col + i]).product();
            let dl = (0..4).map(|i| rows[i][col + 3 - i]).product();
            ans = ans.max(dr).max(dl);
        }
    }

    println!("{}", ans);
}
