use pe_rs::big_int::add;

fn main() {
    let mut a = vec![1];
    let mut b = vec![2];
    let mut ans = 0;

    for _ in 1..1000 {
        let tmp = add(&a, &add(&b, &b));
        a = b;
        b = tmp;
        if add(&a, &b).len() > b.len() {
            ans += 1;
        }
    }

    println!("{ans}");
}
