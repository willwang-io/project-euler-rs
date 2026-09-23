// The straightforward implementation is good enough for me.
// But OEIS A007770 may have some smart insights to count those that ends with 1.

fn check_chain(mut n: usize) -> bool {
    while n != 89 && n != 1 {
        let mut next = 0;
        while n > 0 {
            let r = n % 10;
            next += r * r;
            n /= 10;
        }
        n = next;
    }
    n == 89
}

pub fn run() {
    let mut ans = 0;
    for i in 1..10_000_000 {
        if check_chain(i) {
            ans += 1;
        }
    }
    println!("{ans}");
}
