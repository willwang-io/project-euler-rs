use pe_rs::fetch_input;
use std::collections::HashMap;

fn check(words: &[String]) -> i32 {
    let n = words[0].len() as u32;
    let mut ans = 0;

    let start = (10i32.pow(n - 1) - 1).isqrt() + 1;
    let end = (10i32.pow(n) - 1).isqrt();

    'next_square: for root in start..=end {
        let square = root * root;
        let s = square.to_string();
        let mut char_to_digit = [-1; 26];
        let mut digit_to_char = [-1; 10];

        for (c, d) in words[0].bytes().zip(s.bytes()) {
            let c = (c - b'A') as usize;
            let d = (d - b'0') as usize;

            if char_to_digit[c] != -1 && char_to_digit[c] != d as i32 {
                continue 'next_square;
            }
            if digit_to_char[d] != -1 && digit_to_char[d] != c as i32 {
                continue 'next_square;
            }

            char_to_digit[c] = d as i32;
            digit_to_char[d] = c as i32;
        }

        let mut tmp = vec![];
        for word in words {
            if char_to_digit[(word.as_bytes()[0] - b'A') as usize] == 0 {
                continue;
            }
            let num = word
                .bytes()
                .fold(0, |acc, c| acc * 10 + char_to_digit[(c - b'A') as usize]);
            if num.isqrt() * num.isqrt() == num {
                tmp.push(num);
            }
        }

        if tmp.len() > 1 {
            ans = ans.max(*tmp.iter().max().unwrap());
        }
    }

    ans
}

fn main() {
    let input = fetch_input("0098_words.txt").unwrap();
    let words: Vec<_> = input
        .trim()
        .replace('"', "")
        .split(',')
        .map(str::to_owned)
        .collect();

    let mut cnt: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
    for word in words {
        let mut s = word.to_string().into_bytes();
        s.sort_unstable();
        cnt.entry(s).or_default().push(word);
    }

    let mut ans = 0;
    for (_, words) in cnt.iter().filter(|(_, v)| v.len() != 1) {
        ans = ans.max(check(words));
    }
    println!("{ans}");
}
