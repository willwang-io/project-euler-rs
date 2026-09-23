use pe_rs::fetch_input;

pub fn run() {
    let input = fetch_input("0008.txt").unwrap();
    let s = input.as_bytes();

    let n = s.len();
    let mut ans = 0;
    for i in 13..n {
        let cur = &s[i - 13..i]
            .iter()
            .fold(1, |acc, &x| acc * (x - b'0') as i64);
        ans = ans.max(*cur);
    }
    println!("{}", ans);
}
