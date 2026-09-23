use pe_rs::number_theory::gcd;

pub fn run() {
    let reduce = |a: u64, b: u64| -> (u64, u64) {
        let g = gcd(a, b);
        (a / g, b / g)
    };

    let mut n = 1;
    let mut d = 1;

    for a in 10..100 {
        for b in a + 1..100 {
            if a % 10 == 0 && b % 10 == 0 {
                continue;
            }
            let (p, q, r, s) = (a / 10, a % 10, b / 10, b % 10);
            let cur = reduce(a, b);
            if (p == r && cur == reduce(q, s))
                || (p == s && cur == reduce(q, r))
                || (q == r && cur == reduce(p, s))
                || (q == s && cur == reduce(p, r))
            {
                n *= a;
                d *= b;
            }
        }
    }

    println!("{}", reduce(n, d).1);
}
