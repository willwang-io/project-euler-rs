use pe_rs::number_theory::is_prime;

fn digits_to_num(d: &[usize]) -> u64 {
    let mut ans = 0;
    for i in (0..d.len()).rev() {
        ans = ans * 10 + d[i];
    }
    ans as _
}

fn find_sum(d: usize, pos: &[usize], digits: &mut [usize]) -> u64 {
    if pos.is_empty() {
        let num = digits_to_num(digits);
        if is_prime(num) {
            return num;
        }
        return 0;
    }
    let mut ans = 0;
    for i in 0..10 {
        if i == d {
            continue;
        }
        digits[pos[0]] = i;
        if digits[digits.len() - 1] != 0 {
            ans += find_sum(d, &pos[1..], digits);
        }
    }
    ans
}

fn solve(n: usize) -> u64 {
    let mut position_groups = vec![vec![]; n];
    for i in 1usize..(1 << n) - 1 {
        let mut positions = vec![];
        for j in (0..n).rev() {
            if i & (1 << j) == 0 {
                positions.push(j);
            }
        }
        position_groups[i.count_ones() as usize].push(positions);
    }

    let mut total = 0;
    for d in 0..10 {
        for groups in position_groups.iter().rev() {
            let mut ans = 0;
            for positions in groups {
                let mut digits = vec![d; n];
                ans += find_sum(d, positions, &mut digits);
            }
            if ans != 0 {
                total += ans;
                break;
            }
        }
    }
    total
}

pub fn run() {
    println!("{}", solve(10));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(273700, solve(4));
    }
}
