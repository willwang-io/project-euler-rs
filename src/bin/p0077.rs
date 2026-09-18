use pe_rs::number_theory::sieve_of_eratosthenes;

fn main() {
    let limit = 1000;
    let is_prime = sieve_of_eratosthenes(limit);
    let primes: Vec<_> = (0..limit).filter(|&i| is_prime[i]).collect();

    let mut dp = vec![0i64; limit];
    dp[0] = 1;

    for p in primes {
        for i in p..limit {
            dp[i] += dp[i - p];
        }
    }
    let ans = dp.iter().position(|&x| x > 5000).unwrap();
    println!("{ans}");
}
