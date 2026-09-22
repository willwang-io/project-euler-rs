fn all_cube_perm() -> Vec<Vec<u8>> {
    let mut ans = vec![];
    for a in 0..10 {
        for b in a + 1..10 {
            for c in b + 1..10 {
                for d in c + 1..10 {
                    for e in d + 1..10 {
                        for f in e + 1..10 {
                            let mut cur = vec![a, b, c, d, e, f];
                            if cur.contains(&6) && !cur.contains(&9) {
                                cur.push(9);
                            } else if cur.contains(&9) && !cur.contains(&6) {
                                cur.push(6);
                            }
                            ans.push(cur);
                        }
                    }
                }
            }
        }
    }
    ans
}

fn main() {
    let cube1 = all_cube_perm();
    let cube2 = all_cube_perm();

    let squares: [(u8, u8); 9] = [
        (0, 1),
        (0, 4),
        (0, 9),
        (1, 6),
        (2, 5),
        (3, 6),
        (4, 9),
        (6, 4),
        (8, 1),
    ];

    let ok = |c1: &[u8], c2: &[u8]| -> bool {
        let mut all_pair = vec![];
        for &x in c1 {
            for &y in c2 {
                all_pair.push((x, y));
                all_pair.push((y, x));
            }
        }
        squares.iter().all(|s| all_pair.contains(s))
    };

    let mut ans = 0;
    for i in 0..cube1.len() {
        for j in i + 1..cube2.len() {
            if ok(&cube1[i], &cube2[j]) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
