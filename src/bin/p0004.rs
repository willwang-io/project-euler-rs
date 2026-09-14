fn main() {
    let mut ans = 0;

    for x in 100..1000 {
        for y in 100..1000 {
            let p = x * y;
            let rev_p: i32 = p
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            if rev_p == p {
                ans = ans.max(rev_p);
            }
        }
    }

    println!("{}", ans);
}
