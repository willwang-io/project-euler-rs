use pe_rs::number_theory::continued_fractions::convergent;

fn main() {
    let mut arr = vec![2];

    for i in 1..34 {
        arr.push(1);
        arr.push(i * 2);
        arr.push(1);
    }

    let (num, _) = convergent(0, 100, &arr);
    let ans: i32 = num.into_iter().map(i32::from).sum();
    println!("{ans}");
}
