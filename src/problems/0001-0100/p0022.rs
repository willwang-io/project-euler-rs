use pe_rs::fetch_input;

pub fn run() {
    let input = fetch_input("0022_names.txt").unwrap();

    let cleaned = input.replace('"', "");
    let mut names = cleaned.split(',').collect::<Vec<_>>();

    names.sort_unstable();

    let ans = names.iter().enumerate().fold(0, |acc, (i, name)| {
        acc + name.bytes().map(|b| (b - b'A') as usize + 1).sum::<usize>() * (i + 1)
    });

    println!("{}", ans);
}
