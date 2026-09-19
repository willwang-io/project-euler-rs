use pe_rs::number_theory::continued_fractions::PeriodicCF;

fn main() {
    let mut arr = vec![];

    for i in 1..34 {
        arr.push(1);
        arr.push(i * 2);
        arr.push(1);
    }

    let e_cf = PeriodicCF {
        integer_part: 2,
        period: arr,
    };

    let (num, _) = e_cf.convergent(99);
    let ans: i32 = num.into_iter().map(i32::from).sum();
    println!("{ans}");
}
