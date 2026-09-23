use pe_rs::number_theory::is_prime;
use std::collections::HashMap;

pub fn run() {
    let mut cnt: HashMap<String, Vec<u64>> = HashMap::new();

    for i in 1000..10000 {
        if !is_prime(i) {
            continue;
        }
        let mut s = i.to_string().into_bytes();
        s.sort_unstable();
        let token = String::from_utf8(s).unwrap();
        cnt.entry(token).or_default().push(i);
    }

    for (_, v) in cnt.iter().filter(|(_, v)| v.len() > 2) {
        let n = v.len();
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    if v[k] - v[j] == v[j] - v[i] {
                        println!("{}{}{}", v[i], v[j], v[k]);
                    }
                }
            }
        }
    }
}
