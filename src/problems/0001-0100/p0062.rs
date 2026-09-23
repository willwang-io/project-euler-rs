use std::collections::HashMap;

pub fn run() {
    let mut map: HashMap<[u8; 10], Vec<usize>> = HashMap::new();
    let mut i = 1;
    loop {
        let mut x = i * i * i;
        let mut cnt = [0; 10];
        while x > 0 {
            cnt[x % 10] += 1;
            x /= 10;
        }
        let tmp = map.entry(cnt).or_default();
        tmp.push(i * i * i);
        if tmp.len() == 5 {
            println!("{}", tmp[0]);
            break;
        }
        i += 1;
    }
}
