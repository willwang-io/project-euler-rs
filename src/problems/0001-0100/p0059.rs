use pe_rs::fetch_input;

pub fn run() {
    let input = fetch_input("0059_cipher.txt").unwrap();
    let encrypted = input
        .trim()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    for x in b'a'..=b'z' {
        for y in b'a'..=b'z' {
            for z in b'a'..=b'z' {
                let decrypted = encrypted
                    .iter()
                    .enumerate()
                    .map(|(i, b)| b ^ [x, y, z][i % 3])
                    .collect::<Vec<_>>();

                let text = std::str::from_utf8(&decrypted).unwrap();
                // This is based on a _reasonable_ guess.
                if text.contains("Euler") {
                    println!("{text}");
                    let ans: i32 = decrypted.iter().map(|&x| x as i32).sum();
                    println!("{ans}");
                    break;
                }
            }
        }
    }
}
