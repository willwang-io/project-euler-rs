fn main() {
    let n = 100u64;
    let mut diff = u64::MAX / 2;
    let mut ans = 0;

    for i in 1..=n {
        for j in 1..=100 {
            let a = i * (i + 1) / 2;
            let b = j * (j + 1) / 2;
            let total = a * b;
            let cur_diff = total.abs_diff(2_000_000);
            if cur_diff < diff {
                diff = cur_diff;
                ans = i * j;
            }
        }
    }

    println!("{ans}");
}
