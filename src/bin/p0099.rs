use pe_rs::fetch_input;

fn main() {
    let input = fetch_input("0099_base_exp.txt").unwrap();

    let (line_num, _) = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (base, exp) = line.split_once(',').unwrap();
            let base: f64 = base.parse().unwrap();
            let exp: f64 = exp.parse().unwrap();
            (i, exp * base.log10())
        })
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .unwrap();

    println!("{}", line_num + 1);
}
