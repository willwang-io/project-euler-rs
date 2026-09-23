pub fn run() {
    let mut ans = 0;
    for i in 2..500000 {
        let sum = i
            .to_string()
            .bytes()
            .map(|b| ((b - b'0') as i32).pow(5))
            .sum::<i32>();
        if i == sum {
            ans += i;
        }
    }
    println!("{ans}")
}
