use pe_rs::utility::is_palindrome;

fn main() {
    let rev_num = |mut x: u128| -> u128 {
        let mut ans = 0;
        while x > 0 {
            ans = ans * 10 + x % 10;
            x /= 10;
        }
        ans
    };

    let ok = |mut x: u128| -> bool {
        for _ in 0..49 {
            x = x + rev_num(x);
            let s = x.to_string();
            if is_palindrome(&s) {
                return true;
            }
        }
        false
    };

    let mut ans = 0;
    for x in 1..10_000 {
        if !ok(x) {
            ans += 1;
        }
    }
    println!("{ans}");
}
