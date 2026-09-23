use pe_rs::utility::next_permutation;

pub fn run() {
    let f = |x: (i32, i32, i32)| -> i32 {
        let tmp = format!("{}{}{}", x.0, x.1, x.2);
        tmp.parse::<i32>().unwrap()
    };

    let mut a: Vec<i32> = (1..=10).collect();
    let mut ans = String::from("0");

    loop {
        let branches = [
            (a[0], a[5], a[6]),
            (a[1], a[6], a[7]),
            (a[2], a[7], a[8]),
            (a[3], a[8], a[9]),
            (a[4], a[9], a[5]),
        ];
        let sum = |(x, y, z)| x + y + z;
        if branches.iter().all(|&b| sum(b) == sum(branches[0])) {
            let start = branches
                .iter()
                .enumerate()
                .min_by_key(|&(_, b)| b.0)
                .unwrap()
                .0;
            let mut s = String::new();
            for i in 0..branches.len() {
                s.push_str(&f(branches[(start + i) % branches.len()]).to_string());
            }
            ans = ans.max(s);
        }
        if !next_permutation(&mut a) {
            break;
        }
    }

    println!("{ans}");
}
