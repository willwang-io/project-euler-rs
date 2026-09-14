use pe_rs::utility::next_permutation;

fn main() {
    let mut a = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for _ in 1..1_000_000 {
        next_permutation(&mut a);
    }

    let ans = a.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("");
    println!("{}", ans);
}
