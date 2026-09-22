use std::collections::HashSet;

fn mul_partition(n: usize, smallest: usize) -> Vec<Vec<usize>> {
    let mut ans = vec![vec![n]];
    let mut d = smallest;
    while d * d <= n {
        if n % d == 0 {
            for rest in mul_partition(n / d, d) {
                let mut tmp = vec![d];
                tmp.extend(rest);
                ans.push(tmp);
            }
        }
        d += 1;
    }
    ans
}

fn main() {
    let mut mn_k = [usize::MAX; 12_001];

    for i in 2..24_000 {
        for partition in mul_partition(i, 2) {
            let sum: usize = partition.iter().sum();
            let one_needed = i - sum;
            let k = partition.len() + one_needed;
            if k <= 12_000 {
                mn_k[k] = mn_k[k].min(i);
            }
        }
    }

    let unique: HashSet<_> = mn_k.into_iter().skip(2).collect();
    let unique_sum: usize = unique.iter().sum();
    println!("{unique_sum}");
}
