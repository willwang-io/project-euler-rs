pub fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
    let Some(i) = (0..a.len().saturating_sub(1))
        .rev()
        .find(|&i| a[i] < a[i + 1])
    else {
        a.reverse();
        return false;
    };

    let j = (i + 1..a.len()).rev().find(|&j| a[i] < a[j]).unwrap();
    a.swap(i, j);
    a[i + 1..].reverse();
    true
}
