fn is_special(a: &[u32]) -> bool {
    let n = a.len();
    for mask in 0..3usize.pow(n as u32) {
        let mut m = mask;
        let (mut sum_b, mut sum_c) = (0, 0);
        let (mut cnt_b, mut cnt_c) = (0, 0);
        for &x in a {
            match m % 3 {
                1 => {
                    sum_b += x;
                    cnt_b += 1;
                }
                2 => {
                    sum_c += x;
                    cnt_c += 1;
                }
                _ => {}
            }
            m /= 3;
        }
        if cnt_b == 0 || cnt_c == 0 {
            continue;
        }
        if sum_b == sum_c || (cnt_b > cnt_c && sum_b <= sum_c) || (cnt_c > cnt_b && sum_c <= sum_b)
        {
            return false;
        }
    }
    true
}

fn solve(n: usize) -> Vec<u32> {
    fn dfs(n: usize, arr: &mut Vec<u32>, best: &mut Vec<u32>) {
        let sum: u32 = arr.iter().sum();
        if !best.is_empty() && sum >= best.iter().sum::<u32>() {
            return;
        }
        if arr.len() == n {
            *best = arr.clone();
            return;
        }
        let start = arr.last().copied().unwrap_or(0);
        for i in start + 1..50 {
            arr.push(i);
            if is_special(arr) {
                dfs(n, arr, best);
            }
            arr.pop();
        }
    }

    let mut best = vec![];
    dfs(n, &mut vec![], &mut best);
    best
}

pub fn run() {
    let ans = solve(7);
    for x in ans {
        print!("{x}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(1), vec![1]);
        assert_eq!(solve(2), vec![1, 2]);
        assert_eq!(solve(3), vec![2, 3, 4]);
        assert_eq!(solve(4), vec![3, 5, 6, 7]);
        assert_eq!(solve(5), vec![6, 9, 11, 12, 13]);
        assert_eq!(solve(6), vec![11, 18, 19, 20, 22, 25]);
    }
}
