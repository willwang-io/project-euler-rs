use pe_rs::fetch_input;

fn main() {
    let input = fetch_input("0042_words.txt").unwrap();

    let cleaned = input.replace('"', "");
    let names = cleaned.split(',').collect::<Vec<_>>();

    let vals = names
        .iter()
        .map(|name| name.bytes().map(|b| (b - b'A') as i32 + 1).sum::<_>())
        .collect::<Vec<_>>();
    let mx = vals.iter().max().unwrap();
    let mut triangle_nums = vec![];

    let mut i = 0;
    while i * (i + 1) / 2 < *mx {
        triangle_nums.push(i * (i + 1) / 2);
        i += 1;
    }

    let ans = vals
        .iter()
        .filter(|val| triangle_nums.contains(val))
        .count();
    println!("{ans}");
}
