use pe_rs::number_theory::sieve_of_eratosthenes;
use std::collections::HashSet;

fn dfs(res: Vec<usize>, candidates: HashSet<usize>, adj: &[HashSet<usize>]) -> usize {
    if res.len() == 5 {
        return res.iter().sum();
    }
    let mut ans = usize::MAX;
    for &next in &candidates {
        let next_candidates = candidates
            .intersection(&adj[next])
            .copied()
            .filter(|&v| v > next)
            .collect();

        let mut next_res = res.clone();
        next_res.push(next);
        ans = ans.min(dfs(next_res, next_candidates, adj));
    }
    ans
}

fn main() {
    let is_prime = sieve_of_eratosthenes(100_000_000);
    let p: Vec<_> = (3..10_000).step_by(2).filter(|&i| is_prime[i]).collect();
    let mut adj = vec![HashSet::new(); 100_000];

    for i in 0..p.len() {
        for j in i + 1..p.len() {
            let a = format!("{}{}", p[i], p[j]).parse::<usize>().unwrap();
            let b = format!("{}{}", p[j], p[i]).parse::<usize>().unwrap();
            if is_prime[a] && is_prime[b] {
                adj[p[i]].insert(p[j]);
                adj[p[j]].insert(p[i]);
            }
        }
    }
    let candidates: HashSet<usize> = (0..100_000).filter(|&x| !adj[x].is_empty()).collect();
    let ans = dfs(vec![], candidates, &adj);
    println!("{ans}");
}
