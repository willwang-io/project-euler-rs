fn dist(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)
}

pub fn run() {
    let n = 50;
    let mut all_coord = vec![];
    for x in 0..=n {
        for y in 0..=n {
            if x == 0 && y == 0 {
                continue;
            }
            all_coord.push((x, y));
        }
    }

    let mut ans = 0;
    for i in 0..all_coord.len() {
        for j in i + 1..all_coord.len() {
            let (x1, y1) = all_coord[i];
            let (x2, y2) = all_coord[j];
            let mut sides = [dist(0, 0, x1, y1), dist(0, 0, x2, y2), dist(x1, y1, x2, y2)];
            sides.sort_unstable();
            if sides[0] + sides[1] == sides[2] {
                ans += 1;
            }
        }
    }
    println!("{ans}");
}
