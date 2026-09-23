// Classic backtracking would be good enough, but I wanted to try something new and add a reusable
// solver to my library. Hey, we still have 900 problems to go.
// Reference: https://oi-wiki.org/search/dlx/ (in Chinese)

use pe_rs::fetch_input;

#[derive(Clone, Copy)]
struct Node {
    l: usize,
    r: usize,
    u: usize,
    d: usize,
    row: usize,
    col: usize,
}

pub struct Dlx {
    nodes: Vec<Node>,
    first: Vec<usize>,
    size: Vec<usize>,
}

impl Dlx {
    pub fn build(rows: usize, cols: usize) -> Self {
        let nodes = (0..=cols)
            .map(|c| Node {
                l: if c == 0 { cols } else { c - 1 },
                r: if c == cols { 0 } else { c + 1 },
                u: c,
                d: c,
                row: 0,
                col: c,
            })
            .collect();

        Self {
            nodes,
            first: vec![0; rows + 1],
            size: vec![0; cols + 1],
        }
    }

    pub fn insert(&mut self, row: usize, col: usize) {
        assert!(row > 0 && row < self.first.len());
        assert!(col > 0 && col < self.size.len());

        let i = self.nodes.len();
        let d = self.nodes[col].d;
        self.nodes.push(Node {
            l: i,
            r: i,
            u: col,
            d,
            row,
            col,
        });

        self.nodes[d].u = i;
        self.nodes[col].d = i;
        self.size[col] += 1;

        let f = self.first[row];
        if f == 0 {
            self.first[row] = i;
        } else {
            let r = self.nodes[f].r;
            self.nodes[i].l = f;
            self.nodes[i].r = r;
            self.nodes[r].l = i;
            self.nodes[f].r = i;
        }
    }

    fn remove(&mut self, col: usize) {
        let c = self.nodes[col];
        self.nodes[c.l].r = c.r;
        self.nodes[c.r].l = c.l;

        let mut i = c.d;
        while i != col {
            let mut j = self.nodes[i].r;
            while j != i {
                let n = self.nodes[j];
                self.nodes[n.u].d = n.d;
                self.nodes[n.d].u = n.u;
                self.size[n.col] -= 1;
                j = n.r;
            }
            i = self.nodes[i].d;
        }
    }

    fn recover(&mut self, col: usize) {
        let c = self.nodes[col];

        let mut i = c.u;
        while i != col {
            let mut j = self.nodes[i].l;
            while j != i {
                let n = self.nodes[j];
                self.nodes[n.u].d = j;
                self.nodes[n.d].u = j;
                self.size[n.col] += 1;
                j = n.l;
            }
            i = self.nodes[i].u;
        }

        self.nodes[c.l].r = col;
        self.nodes[c.r].l = col;
    }

    pub fn dance(&mut self, answer: &mut Vec<usize>) -> bool {
        let mut col = self.nodes[0].r;
        if col == 0 {
            return true;
        }

        let mut c = self.nodes[col].r;
        while c != 0 {
            if self.size[c] < self.size[col] {
                col = c;
            }
            c = self.nodes[c].r;
        }

        self.remove(col);
        let mut i = self.nodes[col].d;
        while i != col {
            answer.push(self.nodes[i].row);

            let mut j = self.nodes[i].r;
            while j != i {
                self.remove(self.nodes[j].col);
                j = self.nodes[j].r;
            }

            let found = self.dance(answer);

            let mut j = self.nodes[i].l;
            while j != i {
                self.recover(self.nodes[j].col);
                j = self.nodes[j].l;
            }

            if found {
                self.recover(col);
                return true;
            }

            answer.pop();
            i = self.nodes[i].d;
        }

        self.recover(col);
        false
    }
}

fn solve_sudoku(board: &mut [Vec<u8>]) -> bool {
    let mut solver = Dlx::build(9 * 9 * 9, 4 * 81);

    for (r, row) in board.iter().enumerate() {
        for (c, &given) in row.iter().enumerate() {
            let box_id = (r / 3) * 3 + c / 3;
            for d in 0..9 {
                if given != 0 && given as usize != d + 1 {
                    continue;
                }

                let candidate = (r * 9 + c) * 9 + d + 1;

                for constraint in [
                    1 + r * 9 + c,
                    82 + r * 9 + d,
                    163 + c * 9 + d,
                    244 + box_id * 9 + d,
                ] {
                    solver.insert(candidate, constraint);
                }
            }
        }
    }

    let mut ans = Vec::new();
    if !solver.dance(&mut ans) {
        return false;
    }

    for candidate in ans {
        let id = candidate - 1;
        let d = id % 9;
        let c = (id / 9) % 9;
        let r = id / 81;
        board[r][c] = (d + 1) as u8;
    }
    true
}

pub fn run() {
    let input = fetch_input("0096_sudoku.txt").unwrap();
    let lines: Vec<_> = input
        .lines()
        .filter(|line| !line.starts_with("Grid"))
        .collect();

    let mut ans = 0;
    for puzzle in lines.chunks(9) {
        let mut board: Vec<Vec<_>> = puzzle
            .iter()
            .map(|row| row.bytes().map(|c| c - b'0').collect())
            .collect();
        solve_sudoku(&mut board);
        ans += board[0][0] as i32 * 100 + board[0][1] as i32 * 10 + board[0][2] as i32;
    }
    println!("{ans}");
}
