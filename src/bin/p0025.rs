fn main() {
    let mut a = vec![1];
    let mut b = vec![1];
    let mut idx = 2;

    while b.len() < 1000 {
        let mut x = a.iter();
        let mut y = b.iter();
        let mut c = vec![];
        let mut carry = 0;

        loop {
            let (x, y) = (x.next(), y.next());
            if x.is_none() && y.is_none() && carry == 0 {
                break;
            }
            let sum = x.map_or(0, |&c| c) + y.map_or(0, |&c| c) + carry;
            c.push(sum % 10);
            carry = sum / 10;
        }

        (a, b) = (b, c);
        idx += 1;
    }

    println!("{}", idx);
}
