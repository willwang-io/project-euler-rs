use pe_rs::utility::is_palindrome;

fn main() {
    let mut ans = 0;

    for i in 0..1_000_000 {
        if is_palindrome(&format!("{:b}", i)) && is_palindrome(&i.to_string()) {
            ans += i;
        }
    }

    println!("{ans}");
}
