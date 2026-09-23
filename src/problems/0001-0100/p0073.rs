// Reference: https://people.csail.mit.edu/mip/papers/farey/farey.pdf

fn farey_rank(n: usize, num: usize, den: usize) -> usize {
    let mut a = vec![0; n + 1];
    for i in 0..=n {
        a[i] = (num * i) / den;
    }
    for i in 1..=n {
        for j in (2 * i..=n).step_by(i) {
            a[j] -= a[i];
        }
    }
    a.iter().sum()
}

pub fn run() {
    let left = farey_rank(12000, 1, 3);
    let right = farey_rank(12000, 1, 2);
    println!("{}", right - left - 1);
}
