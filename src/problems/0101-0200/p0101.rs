fn newtons(xs: &[i64]) -> i64 {
    fn f(i: usize, j: usize, x: &[i64]) -> i64 {
        if i == j {
            return x[i];
        }
        let tmp1 = f(i + 1, j, x);
        let tmp2 = f(i, j - 1, x);
        (tmp1 - tmp2) / (j - i) as i64
    }

    let next_x = xs.len() as i64 + 1;
    let mut ans = 0;
    for i in 0..xs.len() {
        let mut cur = f(0, i, xs);
        for j in 1..=i {
            cur *= next_x - j as i64;
        }
        ans += cur;
    }
    ans
}

pub fn run() {
    let f = |n: i64| -> i64 {
        let mut ans = 0;
        for i in 0..11 {
            ans += (-1i64).pow(i) * n.pow(i);
        }
        ans
    };

    let a: Vec<_> = (1..=11).map(f).collect();
    let mut ans = 0;
    for i in 1..a.len() {
        let guessed = newtons(&a[..i]);
        if guessed != a[i] {
            ans += guessed;
        }
    }
    println!("{ans}");
}

#[cfg(test)]
mod tests {
    use crate::selected_problem::newtons;

    #[test]
    fn example() {
        let cube: Vec<_> = (1..=4).map(|i| i * i * i).collect();
        let mut expected = 0;
        for i in 1..cube.len() {
            let guessed = newtons(&cube[..i]);
            if guessed != cube[i] {
                expected += guessed;
            }
        }
        assert_eq!(expected, 74);
    }
}