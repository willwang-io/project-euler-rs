use pe_rs::number_theory::is_prime;
use pe_rs::utility::next_permutation;
use std::collections::{BTreeSet, HashSet};

fn partition(a: &[u64]) -> HashSet<BTreeSet<u64>> {
    fn dfs(a: &[u64], cur_par: &mut BTreeSet<u64>, all_par: &mut HashSet<BTreeSet<u64>>) {
        if a.is_empty() {
            all_par.insert(cur_par.clone());
            return;
        }
        let mut cur = 0;
        for (i, x) in a.iter().enumerate() {
            cur = cur * 10 + x;
            if is_prime(cur) {
                cur_par.insert(cur);
                dfs(&a[i + 1..], cur_par, all_par);
                cur_par.remove(&cur);
            }
        }
    }

    let mut cur_par = BTreeSet::new();
    let mut all_par = HashSet::new();
    dfs(a, &mut cur_par, &mut all_par);
    all_par
}

pub fn run() {
    let mut a: Vec<_> = (1..=9).collect();
    let mut ans = HashSet::new();
    loop {
        ans.extend(partition(&a));
        if !next_permutation(&mut a) {
            break;
        }
    }
    println!("{}", ans.len());
}
