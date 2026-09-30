// Created on Sep 29, 2026
// Useful article: https://www.redblobgames.com/grids/hexagons/

use pe_rs::number_theory::is_prime;

const NEIGHBORS: [(i64, i64, i64); 6] = [
    (0, 1, -1),
    (-1, 1, 0),
    (-1, 0, 1),
    (0, -1, 1),
    (1, -1, 0),
    (1, 0, -1),
];

pub fn run() {
    // This is essentially a guess. I first tried with brute-force starting from 1 until I found the
    // 2000th tile. That was slow, but after observing the output, it seemed that only the tile at
    // "12 o'clock", labeled x, or the tile labeled x - 1 could have PD equal to 3.

    let ans = (1..)
        .flat_map(|r| {
            let x = 2 + 3 * r * (r - 1);
            [x - 1, x]
        })
        .filter(|&x| pd(x) == 3)
        .nth(1999)
        .unwrap();
    println!("{ans}")
}

fn pd(n: i64) -> usize {
    let neighbors = get_neighbor_value(n);
    neighbors
        .iter()
        .map(|&x| x.abs_diff(n))
        .filter(|&x| is_prime(x))
        .count()
}

fn get_neighbor_value(val: i64) -> [i64; 6] {
    let (x, y, z) = val_to_coord(val);
    NEIGHBORS.map(|(dx, dy, dz)| coord_to_val(x + dx, y + dy, z + dz))
}

fn val_to_coord(val: i64) -> (i64, i64, i64) {
    if val == 1 {
        return (0, 0, 0);
    }

    let r = ((((12.0 * val as f64) - 15.0).sqrt() + 3.0) / 6.0).floor() as i64;
    let d = val - (2 + r * (r - 1) * 3);

    match d / r {
        0 => (-d, r, d - r),
        1 => (-r, 2 * r - d, d - r),
        2 => (d - 3 * r, 2 * r - d, r),
        3 => (d - 3 * r, -r, 4 * r - d),
        4 => (r, d - 5 * r, 4 * r - d),
        5 => (6 * r - d, d - 5 * r, -r),
        _ => unreachable!(),
    }
}

fn coord_to_val(x: i64, y: i64, z: i64) -> i64 {
    if x == 0 && y == 0 && z == 0 {
        return 1;
    }
    let r = x.abs().max(y.abs()).max(z.abs());
    2 + r * (r - 1) * 3
        + if y == r {
            -x
        } else if x == -r {
            r + z
        } else if z == r {
            2 * r - y
        } else if y == -r {
            3 * r + x
        } else if x == r {
            4 * r - z
        } else {
            5 * r + y
        }
}

#[cfg(test)]
mod test {
    use super::*;

    const COUNTER_CLOCKWISE_DIR: [(i64, i64, i64); 6] = [
        (-1, 0, 1),
        (0, -1, 1),
        (1, -1, 0),
        (1, 0, -1),
        (0, 1, -1),
        (-1, 1, 0),
    ];

    fn gen_coord(r: i64) -> Vec<(i64, i64, i64, i64)> {
        if r == 0 {
            return vec![(0, 0, 0, 1)];
        }
        let (mut x, mut y, mut z) = (0, r, -r);
        let mut b = r * (r - 1) * 3 + 2;

        let mut arr = vec![];
        for (dx, dy, dz) in COUNTER_CLOCKWISE_DIR {
            for _ in 0..r {
                arr.push((x, y, z, b));
                x += dx;
                y += dy;
                z += dz;
                b += 1;
            }
        }
        arr
    }

    #[test]
    fn test_coord_to_val() {
        for i in 0..=4 {
            let cases = gen_coord(i);
            for (x, y, z, expected_tile_num) in cases {
                assert_eq!(expected_tile_num, coord_to_val(x, y, z), "{x} {y} {z}");
            }
        }
    }

    #[test]
    fn test_val_to_coord() {
        for i in 0..=4 {
            let cases = gen_coord(i);
            for (exp_x, exp_y, exp_z, num) in cases {
                assert_eq!((exp_x, exp_y, exp_z), val_to_coord(num), "{num}");
            }
        }
    }

    #[test]
    fn test_get_neighbors() {
        let expected = vec![2, 3, 4, 5, 6, 7];
        let mut neighbors = get_neighbor_value(1);
        neighbors.sort_unstable();
        assert_eq!(expected, neighbors);

        let expected = vec![6, 7, 16, 18, 33, 34];
        let mut neighbors = get_neighbor_value(17);
        neighbors.sort_unstable();
        assert_eq!(expected, neighbors);
    }
}
