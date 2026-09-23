pub fn run() {
    let f = |mut n: i64| -> i64 {
        let mut cnt = 0;
        while n != 1 {
            if n % 2 == 0 {
                n /= 2;
            } else {
                n = 3 * n + 1;
            }
            cnt += 1;
        }
        cnt
    };

    let mut ans = 0;
    let mut longest = 0;
    for i in 1..1_000_000 {
        let cur = f(i);
        if cur > longest {
            longest = cur;
            ans = i;
        }
    }

    println!("{}", ans);
}
