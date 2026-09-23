pub fn run() {
    let x = [
        "#", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let y = [
        "#",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];

    let z = [
        "#", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let f = |n: usize| -> String {
        let a = n / 100;
        let b = n / 10 % 10;
        let c = n % 10;

        if a != 0 && n % 100 == 0 {
            return format!("{}hundred", x[a]);
        }

        let s = match (b, c) {
            (0, _) => x[c].to_string(),
            (_, 0) => z[b].to_string(),
            (1, _) => y[c].to_string(),
            _ => format!("{}{}", z[b], x[c]),
        };

        if a == 0 {
            s
        } else {
            format!("{}hundredand{}", x[a], s)
        }
    };

    let mut ans = "onethousand".len();
    for i in 1..1000 {
        ans += f(i).len();
    }
    println!("{}", ans);
}
