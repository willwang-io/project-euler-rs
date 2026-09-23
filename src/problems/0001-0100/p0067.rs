use pe_rs::fetch_input;

pub fn run() {
    let input = fetch_input("0067_triangle.txt").unwrap();
    let mut triangle = input
        .split("\n")
        .map(|row| {
            row.split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for i in (0..triangle.len() - 1).rev() {
        for j in 0..triangle[i].len() {
            triangle[i][j] += triangle[i + 1][j].max(triangle[i + 1][j + 1]);
        }
    }

    println!("{}", triangle[0][0]);
}
