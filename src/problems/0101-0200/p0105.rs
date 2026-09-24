use pe_rs::fetch_input;

fn is_special(a: &[u32]) -> bool {
    let n = a.len();
    for mask in 0..3usize.pow(n as u32) {
        let mut m = mask;
        let (mut sum_b, mut sum_c) = (0, 0);
        let (mut cnt_b, mut cnt_c) = (0, 0);
        for &x in a {
            match m % 3 {
                1 => {
                    sum_b += x;
                    cnt_b += 1;
                }
                2 => {
                    sum_c += x;
                    cnt_c += 1;
                }
                _ => {}
            }
            m /= 3;
        }
        if cnt_b == 0 || cnt_c == 0 {
            continue;
        }
        if sum_b == sum_c || (cnt_b > cnt_c && sum_b <= sum_c) || (cnt_c > cnt_b && sum_c <= sum_b)
        {
            return false;
        }
    }
    true
}

pub fn run() {
    let input = fetch_input("0105_sets.txt").unwrap();
    let mut ans = 0;

    for line in input.lines() {
        let nums: Vec<_> = line.split(',').map(|x| x.parse::<u32>().unwrap()).collect();
        if is_special(&nums) {
            ans += nums.iter().sum::<u32>();
        }
    }
    println!("{ans}");
}
