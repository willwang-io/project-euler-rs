use pe_rs::fetch_input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: i64 = i64::MAX / 4;

fn dijkstra(s: usize, dist: &mut [i64], parent: &mut [Option<usize>], adj: &[Vec<(usize, i64)>]) {
    dist.fill(INF);
    parent.fill(None);

    let mut heap = BinaryHeap::new();

    dist[s] = 0;
    heap.push(Reverse((0_i64, s)));

    while let Some(Reverse((current_dist, v))) = heap.pop() {
        if current_dist != dist[v] {
            continue;
        }

        for &(to, weight) in &adj[v] {
            let new_dist = current_dist + weight;

            if new_dist < dist[to] {
                dist[to] = new_dist;
                parent[to] = Some(v);
                heap.push(Reverse((new_dist, to)));
            }
        }
    }
}

pub fn run() {
    let input = fetch_input("0083_matrix.txt");
    let mat = input
        .unwrap()
        .replace('\n', ",")
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let n = mat.len();
    let m = n.isqrt();
    let mut adj = vec![vec![]; n];

    for i in 0..n {
        let col = i % m;
        let neighbors = [
            (col > 0).then(|| i - 1),
            (col + 1 < m).then(|| i + 1),
            (i >= m).then(|| i - m),
            (i + m < n).then(|| i + m),
        ];
        for next in neighbors.into_iter().flatten() {
            adj[i].push((next, mat[next]));
        }
    }

    let mut dist = vec![INF; n];
    let mut parent = vec![None; n];
    dijkstra(0, &mut dist, &mut parent, &adj);
    println!("{}", dist[n - 1] + mat[0]);
}
