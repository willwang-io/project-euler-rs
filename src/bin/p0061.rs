use std::collections::HashSet;

fn main() {
    let mut arr = vec![];
    for i in 1000..10000 {
        for (bit, matches) in [
            is_tri(i),
            is_sqrt(i),
            is_pen(i),
            is_hex(i),
            is_hep(i),
            is_oct(i),
        ]
        .into_iter()
        .enumerate()
        {
            if matches {
                arr.push((1 << bit, i))
            }
        }
    }

    let mut adj = vec![HashSet::new(); 100];
    for (_, x) in &arr {
        for y in &arr {
            if x % 100 == y.1 / 100 {
                adj[(x % 100) as usize].insert(y);
            }
        }
    }

    fn dfs(u: i32, mask: i32, cur: &mut Vec<i32>, adj: &[HashSet<&(i32, i32)>]) -> bool {
        if cur.len() == 6 {
            if cur[0] / 100 == cur[5] % 100 && mask == (1 << 6) - 1 {
                println!("{}", cur.iter().sum::<i32>());
                return true;
            }
            return false;
        }
        for &&(b, v) in &adj[(u % 100) as usize] {
            if mask & b != 0 || cur.contains(&v) {
                continue;
            }
            cur.push(v);
            if dfs(v, mask | b, cur, adj) {
                return true;
            }
            cur.pop();
        }
        false
    }

    for &(b, u) in &arr {
        let mut cur = vec![u];
        if dfs(u, b, &mut cur, &adj) {
            break;
        }
    }
}

fn is_tri(k: i32) -> bool {
    let x = 1 + 8 * k;
    let x_sqrt = x.isqrt();
    x_sqrt * x_sqrt == x && (x - 1) % 2 == 0
}

fn is_sqrt(k: i32) -> bool {
    k.isqrt() * k.isqrt() == k
}

fn is_pen(k: i32) -> bool {
    let x = 1 + 24 * k;
    let x_sqrt = x.isqrt();
    x_sqrt * x_sqrt == x && (1 + x_sqrt) % 6 == 0
}

fn is_hex(k: i32) -> bool {
    let x = 1 + 8 * k;
    let x_sqrt = x.isqrt();
    x_sqrt * x_sqrt == x && (1 + x_sqrt) % 4 == 0
}

fn is_hep(k: i32) -> bool {
    let x = 9 + 40 * k;
    let x_sqrt = x.isqrt();
    x_sqrt * x_sqrt == x && (3 + x_sqrt) % 10 == 0
}

fn is_oct(k: i32) -> bool {
    let x = 4 + 12 * k;
    let x_sqrt = x.isqrt();
    x_sqrt * x_sqrt == x && (2 + x_sqrt) % 6 == 0
}
