pub fn run() {
    let ans = (1..1000).fold(0, |acc, x| {
        acc + if x % 3 == 0 || x % 5 == 0 { x } else { 0 }
    });
    println!("{ans}");
}
