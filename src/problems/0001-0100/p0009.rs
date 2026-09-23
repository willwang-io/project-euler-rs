pub fn run() {
    for a in 1..500 {
        for b in a + 1..500 {
            let c = 1000 - a - b;
            if c <= 0 {
                break;
            }
            if a * a + b * b == c * c {
                println!("{}", a * b * c);
            }
        }
    }
}
