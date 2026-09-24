fn count_tests(a: &[u32]) -> usize {
    let n = a.len();
    let mut cnt = 0;
    for mask in 0..3usize.pow(n as u32) {
        let mut m = mask;
        let (mut b, mut c) = (Vec::new(), Vec::new());

        for &x in a {
            match m % 3 {
                1 => b.push(x),
                2 => c.push(x),
                _ => {}
            }
            m /= 3;
        }

        if b.is_empty() || b.len() != c.len() {
            continue;
        }

        let mut less = false;
        let mut greater = false;
        for (&x, &y) in b.iter().zip(&c) {
            less |= x < y;
            greater |= x > y;
        }

        if less && greater {
            cnt += 1;
        }
    }

    cnt / 2
}

pub fn run() {
    println!("{}", count_tests(&(0..12).collect::<Vec<_>>()));
}

#[cfg(test)]
mod test {
    use crate::selected_problem::count_tests;

    #[test]
    fn example() {
        assert_eq!(1, count_tests(&(0..4).collect::<Vec<_>>()));
        assert_eq!(70, count_tests(&(0..7).collect::<Vec<_>>()));
    }
}
