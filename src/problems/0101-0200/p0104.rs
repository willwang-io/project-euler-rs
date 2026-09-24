fn first_nine(i: usize) -> usize {
    let sqrt5 = 5f64.sqrt();
    let phi = (1.0 + sqrt5) / 2.0;
    let log = i as f64 * phi.log10() - sqrt5.log10();
    10f64.powf(log.fract() + 8.0).floor() as usize
}

fn ok(mut n: usize) -> bool {
    let mut cnt = [0; 10];
    while n > 0 {
        cnt[n % 10] += 1;
        n /= 10;
    }
    cnt[0] == 0 && cnt.iter().skip(1).all(|&x| x == 1)
}

pub fn run() {
    let mut a = 1;
    let mut b = 1;
    let mut i = 3;

    loop {
        (a, b) = (b, (a + b) % 1_000_000_000);
        if ok(b) && ok(first_nine(i)) {
            println!("{i}");
            break;
        }
        i += 1;
    }
}
