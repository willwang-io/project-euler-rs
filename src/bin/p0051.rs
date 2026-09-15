use pe_rs::number_theory::sieve_of_eratosthenes;

fn main() {
    let s = sieve_of_eratosthenes(1_000_000);

    let ok = |x: u64| -> (usize, u64) {
        let x = x
            .to_string()
            .bytes()
            .map(|b| (b - b'0') as u64)
            .collect::<Vec<_>>();
        let n = x.len();

        let mut family_size = 0;
        let mut ans = 0;

        for mask in 1..1 << n {
            let mut cnt = 0;
            let mut first = 0;

            let mut base = 0;
            let mut temp = 0;
            for (i, &d) in x.iter().enumerate() {
                base *= 10;
                temp *= 10;
                if mask & (1 << i) != 0 {
                    temp += 1;
                } else {
                    base += d;
                }
            }

            for j in 0..10 {
                if j == 0 && mask & 1 != 0 {
                    continue;
                }

                let num = base + j * temp;
                if s[num as usize] {
                    if cnt == 0 {
                        first = num;
                    }
                    cnt += 1;
                }
            }
            if cnt > family_size {
                family_size = cnt;
                ans = first;
            }
        }
        (family_size, ans)
    };

    for i in 1..1000000 {
        if !s[i as usize] {
            continue;
        }
        let (sz, p) = ok(i);
        if sz == 8 {
            println!("{p}");
            break;
        }
    }
}
