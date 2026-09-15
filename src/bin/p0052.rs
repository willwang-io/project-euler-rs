fn main() {
    let mut x = 1;
    loop {
        let mut chars = x.to_string().chars().collect::<Vec<_>>();
        chars.sort_unstable();
        let mut ok = true;

        for i in 2..7 {
            let mut tmp = (x * i).to_string().chars().collect::<Vec<_>>();
            tmp.sort_unstable();
            if tmp != chars {
                ok = false;
            }
        }

        if ok {
            println!("{x}");
            break;
        }

        x += 1;
    }
}
