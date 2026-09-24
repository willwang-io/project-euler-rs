use pe_rs::fetch_input;

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let p = self.parent[x];
            self.parent[x] = self.find(p);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut a = self.find(a);
        let mut b = self.find(b);

        if a == b {
            return false;
        }

        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }

        self.parent[b] = a;
        self.size[a] += self.size[b];
        self.components -= 1;
        true
    }

    fn components(&self) -> usize {
        self.components
    }
}

struct Edge {
    u: usize,
    v: usize,
    w: i32,
}

fn kruskal(a: &[Vec<i32>]) -> Option<i32> {
    let n = a.len();
    let mut edges = vec![];

    for i in 0..n {
        for j in i + 1..n {
            if a[i][j] != 0 {
                edges.push(Edge {
                    u: i,
                    v: j,
                    w: a[i][j],
                })
            }
        }
    }

    edges.sort_by(|a, b| a.w.cmp(&b.w));

    let mut dsu = Dsu::new(n);
    let mut ans = 0;

    for edge in edges {
        if dsu.union(edge.u, edge.v) {
            ans += edge.w;
        }
    }

    if dsu.components() == 1 {
        Some(ans)
    } else {
        None
    }
}

fn solve(input: String) -> i32 {
    let mat: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<i32>().unwrap_or(0))
                .collect()
        })
        .collect();

    let mut total = 0;
    for i in 0..mat.len() {
        for j in i + 1..mat.len() {
            total += mat[i][j];
        }
    }

    if let Some(min_w) = kruskal(&mat) {
        return total - min_w;
    }

    -1
}

pub fn run() {
    let input = fetch_input("0107_network.txt").unwrap();
    let ans = solve(input);
    println!("{ans}");
}

#[cfg(test)]
mod test {
    use crate::selected_problem::solve;

    #[test]
    fn example() {
        let network = String::from(
            "-,16,12,21,-,-,-\n\
     16,-,-,17,20,-,-\n\
     12,-,-,28,-,31,-\n\
     21,17,28,-,18,19,23\n\
     -,20,-,18,-,-,11\n\
     -,-,31,19,-,-,27\n\
     -,-,-,23,11,27,-",
        );

        assert_eq!(150, solve(network));
    }
}
