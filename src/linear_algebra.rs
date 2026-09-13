pub fn mat_mul(a: &[Vec<u64>], b: &[Vec<u64>], modules: u64) -> Vec<Vec<u64>> {
    let n = a.len();
    let k = b.len();
    let m = b[0].len();

    assert_eq!(a[0].len(), k);

    let mut result = vec![vec![0u64; m]; n];
    for i in 0..n {
        for j in 0..k {
            if a[i][j] == 0 {
                continue;
            }
            let a_val = a[i][j] as u128;
            for l in 0..m {
                result[i][l] =
                    ((result[i][l] as u128 + a_val * b[j][l] as u128) % modules as u128) as u64;
            }
        }
    }
    result
}

pub fn mat_pow(m: &[Vec<u64>], exp: u64, modules: u64) -> Vec<Vec<u64>> {
    let n = m.len();
    let mut result: Vec<Vec<u64>> = vec![vec![0u64; n]; n];
    for i in 0..n {
        result[i][i] = 1;
    }
    let mut base: Vec<Vec<u64>> = m.to_vec();
    let mut e = exp;

    while e > 0 {
        if e & 1 == 1 {
            result = mat_mul(&result, &base, modules);
        }
        base = mat_mul(&base, &base, modules);
        e >>= 1;
    }

    result
}

pub fn gaussian_elimination(mut matrix: Vec<Vec<f64>>) -> Option<Vec<f64>> {
    let n = matrix.len();
    for col in 0..n {
        let pivot_row = (col..n)
            .max_by(|&a, &b| {
                matrix[a][col]
                    .abs()
                    .partial_cmp(&matrix[b][col].abs())
                    .unwrap()
            })
            .unwrap();

        matrix.swap(col, pivot_row);
        let pivot = matrix[col][col];
        if pivot.abs() < 1e-10 {
            return None;
        }

        for row in (col + 1)..n {
            let factor = matrix[row][col] / pivot;
            for j in col..=n {
                let val = matrix[col][j] * factor;
                matrix[row][j] -= val;
            }
        }
    }
    let mut sol = vec![0.0; n];
    for row in (0..n).rev() {
        let rhs = matrix[row][n];
        let sum: f64 = ((row + 1)..n).map(|j| matrix[row][j] * sol[j]).sum();
        sol[row] = (rhs - sum) / matrix[row][row];
    }
    Some(sol)
}
