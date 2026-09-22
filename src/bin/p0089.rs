use pe_rs::fetch_input;

fn main() {
    let input = fetch_input("0089_roman.txt").unwrap();
    let roman: Vec<String> = input.lines().map(str::to_owned).collect::<Vec<_>>();

    let roman_to_num = |s: &String| -> usize {
        let nums: Vec<usize> = s
            .chars()
            .map(|c| match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => unreachable!(),
            })
            .collect();

        nums.iter().enumerate().fold(0, |acc, (i, x)| {
            let mut ans = acc + x;
            if i > 0 && nums[i] > nums[i - 1] {
                ans -= nums[i - 1] * 2;
            }
            ans
        })
    };

    let num_to_roman = |num: usize| -> String {
        let ones = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
        let tens = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
        let hundreds = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
        format!(
            "{}{}{}{}",
            "M".repeat(num / 1000),
            hundreds[num / 100 % 10],
            tens[num / 10 % 10],
            ones[num % 10]
        )
    };

    let mut ans = 0;
    for roman in roman {
        let num = roman_to_num(&roman);
        let best_roman = num_to_roman(num);
        ans += roman.len().saturating_sub(best_roman.len());
    }
    println!("{ans}");
}
