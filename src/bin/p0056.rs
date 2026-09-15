use pe_rs::big_int::mul;

fn main() {
    let mut ans = 0;
    for a in 1..100 {
        let bytes_a: Vec<u8> = a.to_string().bytes().map(|b| b - b'0').collect();
        let mut cur = bytes_a.clone();
        for _ in 1..100 {
            cur = mul(&cur, &bytes_a);
            let mut sum = 0;
            for &x in &cur {
                sum += x as i32;
            }
            ans = ans.max(sum);
        }
    }
    println!("{ans}");
}
