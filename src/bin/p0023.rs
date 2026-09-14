use pe_rs::number_theory::divisors;

fn main() {
    let proper_divisor_sum = |x: u64| -> u64 {
        let d = divisors(x);
        d.iter().sum::<u64>() - x
    };

    let abundant = (0..=28123)
        .filter(|&i| proper_divisor_sum(i) > i)
        .collect::<Vec<_>>();

    let limit = 28123;
    let mut arr = vec![false; limit + 1];

    for (i, &a) in abundant.iter().enumerate() {
        for &b in &abundant[i..] {
            let sum = (a + b) as usize;
            if sum > limit {
                break;
            }
            arr[sum] = true;
        }
    }

    let ans: usize = (1..=limit).filter(|&n| !arr[n]).sum();
    println!("{}", ans);
}
