use pe_rs::number_theory::factorial;

pub fn run() {
    let helper = |mut x: u64| -> u64 {
        let mut ans = 0;
        while x > 0 {
            ans += factorial(x % 10);
            x /= 10;
        }
        ans
    };

    let ans = (3..50000).fold(0, |acc, x| {
        acc + if helper(x) == x {
            println!("{x}");
            x
        } else {
            0
        }
    });
    println!("{ans}");
}
