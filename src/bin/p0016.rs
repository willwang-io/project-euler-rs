fn main() {
    let mut a = vec![1];

    for _ in 0..1000 {
        let mut b = a.iter();
        let mut carry = 0;
        let mut cur = vec![];

        loop {
            let x = b.next();
            if x.is_none() && carry == 0 {
                break;
            }
            let sum = x.map_or(0, |&c| c) * 2 + carry;
            cur.push(sum % 10);
            carry = sum / 10;
        }
        a = cur;
    }

    let ans: i32 = a.iter().sum();
    println!("{}", ans);
}
