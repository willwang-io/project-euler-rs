pub fn run() {
    const N: usize = 40;
    const SIDES: usize = 4;

    // mat[x][cnt1][y][cnt2] reads as from cell x with cnt1 consecutive doubles,
    // to cell y with cnt2 consecutive doubles.
    let mut mat = [[[[0.0; 3]; N]; 3]; N];

    for x in 0..N {
        for cnt1 in 0..3 {
            for die1 in 1..=SIDES {
                for die2 in 1..=SIDES {
                    let (next_cell, next_cnt) = if die1 == die2 {
                        if cnt1 == 2 {
                            (10, 0)
                        } else {
                            ((x + die1 + die2) % N, cnt1 + 1)
                        }
                    } else {
                        ((x + die1 + die2) % N, 0)
                    };
                    mat[x][cnt1][next_cell][next_cnt] += 1.0 / (SIDES * SIDES) as f64;
                }
            }
        }
    }

    // G2J
    for x in 0..N {
        for cnt1 in 0..3 {
            for cnt2 in 0..3 {
                mat[x][cnt1][10][0] += mat[x][cnt1][30][cnt2];
                mat[x][cnt1][30][cnt2] = 0.0;
            }
        }
    }

    // CH
    for x in 0..N {
        for cnt1 in 0..3 {
            for ch in [7, 22, 36] {
                for cnt2 in 0..3 {
                    let p = mat[x][cnt1][ch][cnt2];
                    mat[x][cnt1][ch][cnt2] = 6.0 * p / 16.0;

                    mat[x][cnt1][0][cnt2] += p / 16.0; // GO
                    mat[x][cnt1][10][0] += p / 16.0; // JAIL
                    mat[x][cnt1][11][cnt2] += p / 16.0; // C1
                    mat[x][cnt1][24][cnt2] += p / 16.0; // E3
                    mat[x][cnt1][39][cnt2] += p / 16.0; // H2
                    mat[x][cnt1][5][cnt2] += p / 16.0; // R1

                    // Go to next R
                    let next_r = match ch {
                        7 => 15,
                        22 => 25,
                        36 => 5,
                        _ => unreachable!(),
                    };
                    mat[x][cnt1][next_r][cnt2] += 2.0 * p / 16.0;

                    // Go to next U
                    let next_u = if ch == 22 { 28 } else { 12 };
                    mat[x][cnt1][next_u][cnt2] += p / 16.0;

                    // Go back 3 squares
                    mat[x][cnt1][ch - 3][cnt2] += p / 16.0;
                }
            }
        }
    }

    // CC
    for x in 0..N {
        for cnt1 in 0..3 {
            for cc in [2, 17, 33] {
                for cnt2 in 0..3 {
                    let p = mat[x][cnt1][cc][cnt2];
                    mat[x][cnt1][cc][cnt2] = 14.0 * p / 16.0;

                    mat[x][cnt1][0][cnt2] += p / 16.0;
                    mat[x][cnt1][10][0] += p / 16.0;
                }
            }
        }
    }

    let mut prob = [[0.0f64; 3]; N];
    prob[0][0] = 1.0;

    loop {
        let mut next = [[0.0f64; 3]; N];
        for x in 0..N {
            for cnt1 in 0..3 {
                for y in 0..N {
                    for cnt2 in 0..3 {
                        next[y][cnt2] += prob[x][cnt1] * mat[x][cnt1][y][cnt2];
                    }
                }
            }
        }

        let mut diff = 0.0f64;
        for x in 0..N {
            for cnt in 0..3 {
                diff = diff.max((next[x][cnt] - prob[x][cnt]).abs());
            }
        }

        prob = next;
        if diff <= 1e-14 {
            break;
        }
    }

    let mut cells: Vec<(usize, f64)> = (0..N).map(|x| (x, prob[x].iter().sum::<f64>())).collect();
    cells.sort_by(|a, b| b.1.total_cmp(&a.1));

    for (cell, _) in &cells[..3] {
        print!("{cell:02}");
    }
}
