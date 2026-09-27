fn solve(n: usize) -> f64 {
    fn dfs(n: usize, red_limit: usize) -> f64 {
        if n == 0 {
            return 1.0;
        }

        let blue_prob = 1.0 / (n + 1) as f64;
        if red_limit == 0 {
            return blue_prob * dfs(n - 1, red_limit);
        }
        let red_prob = 1.0 - blue_prob;
        blue_prob * dfs(n - 1, red_limit) + red_prob * dfs(n - 1, red_limit - 1)
    }

    let prob = dfs(n, (n - 1) / 2);
    let prob_inv = prob.powf(-1.0);
    prob_inv.floor()
}

pub fn run() {
    println!("{}", solve(15));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(10.0, solve(4));
    }
}
