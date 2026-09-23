use pe_rs::number_theory::sieve_of_eratosthenes;

pub fn run() {
    let limit = 1_000_000;
    let s = sieve_of_eratosthenes(limit);

    let mut p_sum = vec![0];
    let mut sum = 0;
    for i in 0..limit {
        if sum + i > 1_000_000 {
            break;
        }
        if s[i] {
            sum += i;
            p_sum.push(sum);
        }
    }

    let mut longest = 0;
    let mut ans = 0;
    for i in 1..p_sum.len() {
        for j in 0..i {
            let tmp = p_sum[i] - p_sum[j];
            if s[tmp] && i - j > longest {
                longest = i - j;
                ans = tmp;
            }
        }
    }
    println!("{ans}");
}
