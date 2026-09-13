use std::process::Command;

fn fetch_input() -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let output = Command::new("curl")
        .args([
            "-fLaS",
            "https://projecteuler.net/resources/documents/1001_input.txt",
        ])
        .output()?;

    let text = String::from_utf8(output.stdout)?;

    let arr = text
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    Ok(arr)
}

fn contains_or_not_overlaps(l1: usize, r1: usize, l2: usize, r2: usize) -> bool {
    let mut tmp = vec![(l1, r1), (l2, r2)];
    tmp.sort_unstable();
    tmp[0].1 < tmp[1].0 || (tmp[0].0 < tmp[1].0 && tmp[1].1 < tmp[0].1)
}

fn dfs(i: usize, l: usize, r: usize, left: &[usize], right: &[usize]) -> i32 {
    if i == left.len() {
        return 0;
    }
    let v = dfs(i + 1, l, r, left, right);
    let mut ans = v;
    if contains_or_not_overlaps(l, r, left[i], right[i]) {}
    ans + dfs(i + 1, left[i], right[i], left, right)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let arr = fetch_input()?;

    let arr = vec![0, 1, 2, 3, 1, 4, 0, 5, 4, 2, 6, 7, 3, 8, 6, 5, 9, 8, 9, 7];
    let arr = vec![0, 0, 1, 1];

    let n = arr.len() / 2;
    let mut left = vec![0; n];
    let mut right = vec![0; n];

    for i in 0..2 * n {
        right[arr[i]] = i + 1;
    }

    for i in (0..2 * n).rev() {
        left[arr[i]] = i + 1;
    }

    let ans = dfs(0, 0, 2 * n + 1, &left, &right);
    println!("{}", ans);

    Ok(())
}
