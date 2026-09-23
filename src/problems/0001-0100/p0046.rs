use pe_rs::number_theory::sieve_of_eratosthenes;

pub fn run() {
    let limit = 100_000;
    let s = sieve_of_eratosthenes(limit);
    let mut odd = vec![false; limit];

    for i in (3..limit).step_by(2) {
        if !s[i] {
            continue;
        }
        for j in 1..50 {
            let tmp = i + 2 * j * j;
            if tmp < limit {
                odd[tmp] = true;
            }
        }
    }

    for i in (3..limit).step_by(2) {
        if !s[i] && !odd[i] {
            println!("{i}");
            break;
        }
    }
}
