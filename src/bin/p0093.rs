use std::collections::HashSet;

fn dfs(nums: [f64; 4], n: usize, seen: &mut HashSet<u32>) {
    if n == 1 {
        let val = nums[0];
        let rounded = val.round();
        if rounded > 0.0 && (val - rounded).abs() < 1e-9 {
            seen.insert(rounded as u32);
        }
        return;
    }
    for i in 0..n {
        for j in i + 1..n {
            let (x, y) = (nums[i], nums[j]);
            let mut next = nums;
            next[j] = nums[n - 1];

            for val in [x + y, x - y, y - x, x * y, x / y, y / x] {
                if !val.is_finite() {
                    continue;
                }
                next[i] = val;
                dfs(next, n - 1, seen);
            }
        }
    }
}

fn main() {
    let mut longest = 0;
    let mut ans = [0; 4];
    let mut seen = HashSet::new();

    for a in 1..10 {
        for b in a + 1..10 {
            for c in b + 1..10 {
                for d in c + 1..10 {
                    seen.clear();
                    dfs([a as f64, b as f64, c as f64, d as f64], 4, &mut seen);

                    let mut i = 0;
                    while seen.contains(&(i + 1)) {
                        i += 1;
                    }

                    if i > longest {
                        longest = i;
                        ans = [a, b, c, d];
                    }
                }
            }
        }
    }

    let [a, b, c, d] = ans;
    println!("{a}{b}{c}{d}");
}
