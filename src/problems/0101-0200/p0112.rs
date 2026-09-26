fn not_bouncy(x: u64) -> bool {
    let s = x.to_string();
    let b = s.as_bytes();
    let forward = b.windows(2).all(|w| w[0] <= w[1]);
    let backward = b.windows(2).all(|w| w[0] >= w[1]);
    forward || backward
}

fn solve(percent: u64) -> u64 {
    let mut bouncy_cnt = 0;
    for i in 1.. {
        if !not_bouncy(i) {
            bouncy_cnt += 1;
        }
        if bouncy_cnt * 100 == percent * i {
            return i;
        }
    }
    0
}

pub fn run() {
    println!("{}", solve(99));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(538, solve(50));
        assert_eq!(21780, solve(90));
    }
}
