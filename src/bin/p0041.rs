use pe_rs::number_theory::is_prime;
use pe_rs::utility::next_permutation;

fn main() {
    let mut ans = 0;
    for n in (1..10).rev() {
        let mut arr: Vec<u64> = (1..=n).collect();

        loop {
            let mut tmp = 0;
            for x in &arr {
                tmp = tmp * 10 + *x;
            }
            if is_prime(tmp) {
                ans = ans.max(tmp);
            }
            if !next_permutation(&mut arr) {
                break;
            }
        }
    }

    println!("{ans}");
}
