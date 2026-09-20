use pe_rs::fetch_input;
use std::collections::VecDeque;

fn topological_sort(adj: &[Vec<usize>]) -> Option<Vec<usize>> {
    let n = adj.len();
    let mut indegree = vec![0; n];

    for edges in adj {
        for &v in edges {
            indegree[v] += 1;
        }
    }

    let mut q = VecDeque::new();

    for i in 0..n {
        if indegree[i] == 0 {
            q.push_back(i);
        }
    }

    let mut order = Vec::with_capacity(n);

    while let Some(u) = q.pop_front() {
        order.push(u);

        for &v in &adj[u] {
            indegree[v] -= 1;

            if indegree[v] == 0 {
                q.push_back(v);
            }
        }
    }

    if order.len() == n { Some(order) } else { None }
}

fn main() {
    let input = fetch_input("0079_keylog.txt").unwrap();
    let key_log = input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| (b - b'0') as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut adj = vec![vec![]; 10];
    let mut seen = [false; 10];

    for log in key_log {
        for &d in &log {
            seen[d] = true;
        }
        adj[log[0]].push(log[1]);
        adj[log[1]].push(log[2]);
    }

    if let Some(order) = topological_sort(&adj) {
        let ans: String = order
            .into_iter()
            .filter(|&d| seen[d])
            .map(|d| (b'0' + d as u8) as char)
            .collect();
        println!("{ans}");
    }
}
