use pe_rs::number_theory::is_prime;

fn main() {
    let mut cnt = 0;
    let mut i = 3;
    loop {
        let c1 = i * i;
        let c2 = c1 - i + 1;
        let c3 = c2 - i + 1;
        let c4 = c3 - i + 1;
        cnt += [c1, c2, c3, c4].iter().filter(|&&x| is_prime(x)).count();
        if cnt * 10 < (i * 2 - 1) as usize {
            println!("{i}");
            break;
        }
        i += 2;
    }
}
