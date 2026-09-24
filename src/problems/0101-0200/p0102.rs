use pe_rs::fetch_input;

fn to_left(p: (i32, i32), q: (i32, i32)) -> bool {
    let area = p.0 * q.1 - p.1 * q.0;
    area > 0
}

fn origin_inside_triangle(x: (i32, i32), y: (i32, i32), z: (i32, i32)) -> bool {
    (to_left(x, y) && to_left(y, z) && to_left(z, x))
        || (to_left(y, x) && to_left(z, y) && to_left(x, z))
}

pub fn run() {
    let input = fetch_input("0102_triangles.txt").unwrap();

    let triangles: Vec<Vec<_>> = input
        .lines()
        .map(|row| row.split(',').map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    let ans = triangles
        .iter()
        .filter(|pts| origin_inside_triangle((pts[0], pts[1]), (pts[2], pts[3]), (pts[4], pts[5])))
        .count();

    println!("{ans}");
}

#[cfg(test)]
mod tests {
    use crate::selected_problem::origin_inside_triangle;

    #[test]
    fn example() {
        let a = (-340, 495);
        let b = (-153, -910);
        let c = (835, -947);
        assert!(origin_inside_triangle(a, b, c));

        let x = (-175, 41);
        let y = (-421, -714);
        let z = (574, -645);
        assert!(!origin_inside_triangle(x, y, z));
    }
}
