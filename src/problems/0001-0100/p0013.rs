use pe_rs::fetch_input;

pub fn run() {
    let input = fetch_input("0013.txt").unwrap();
    let nums = input.split('\n').collect::<Vec<_>>();

    let ans = nums.iter().fold(String::from("0"), |acc, x| {
        let mut a = acc.bytes().rev();
        let mut b = x.bytes().rev();
        let mut carry = 0;
        let mut ans = vec![];

        loop {
            let (x, y) = (a.next(), b.next());
            if x.is_none() && y.is_none() && carry == 0 {
                break;
            }
            let sum = x.map_or(0, |c| c - b'0') + y.map_or(0, |c| c - b'0') + carry;
            ans.push(b'0' + sum % 10);
            carry = sum / 10;
        }

        ans.reverse();
        String::from_utf8(ans).unwrap()
    });

    println!("{}", &ans[..10]);
}
