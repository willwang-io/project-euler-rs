pub fn run() {
    let mut ans = 1;

    for i in 2..7 {
        let start_num = 10u32.pow(i - 1);
        let target_idx = 10u32.pow(i) - 1;
        let block_start_idx = (1 + (9 * i - 10) * 10u32.pow(i - 1)) / 9;

        let num = start_num + (target_idx - block_start_idx) / i;
        let offset = (target_idx - block_start_idx) % i;
        let tmp = num.to_string().as_bytes()[offset as usize] - b'0';
        ans *= tmp;
    }

    println!("{ans}");
}
