fn main() {
    let mut ans = String::from("0");

    for i in 1..10000 {
        let mut s = String::new();
        for n in 1..50 {
            s = format!("{s}{}", i * n);
            if s.len() >= 9 {
                break;
            }
        }
        let mut cnt = [0; 10];
        for b in s.bytes() {
            cnt[(b - b'0') as usize] += 1;
        }
        if cnt[0] == 0 && cnt[1..].iter().all(|&x| x == 1) {
            ans = ans.max(s);
        }
    }

    println!("{ans}");
}
