use pe_rs::number_theory::{is_prime, n_order};

#[test]
fn test_is_prime() {
    for (n, expected) in [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        // Base 2 passes after one square; witnesses divisible by 5 are skipped.
        (5, true),
        // Base 2 initially gives 1: advance to the next witness.
        (7, true),
        // Base 2 follows 2 -> 4 -> 7, exhausting the squares without reaching 8.
        (9, false),
        // Base 2 initially gives n - 1: no squaring needed.
        (11, true),
        // s = 1 and the initial residue is 8: reject without any squares.
        (15, false),
        // Base 450775 follows 3 -> 9 -> 13 -> 16, passing on the last square.
        (17, true),
        (25, false),
        // The first two witnesses pass after squaring; base 9375 initially gives 1.
        (41, true),
        (49, false),
        (97, true),
        (99, false),
        (121, false),
        // 23 * 89: base 2 initially gives 1, but base 325 rejects it.
        (2_047, false),
        // 829 * 1657: base 2 passes after squaring, but base 325 rejects it.
        (1_373_653, false),
    ] {
        assert_eq!(is_prime(n), expected, "n = {n}");
    }
}

#[test]
fn test_n_order() {
    for (a, n, expected) in [
        (3, 7, Some(6)),
        (4, 7, Some(3)),
        (10, 7, Some(6)),
        (10, 27, Some(3)),
        (1, 2, Some(1)),
        (15, 7, Some(1)),
        (11, 7, Some(3)),
        (3, 8, Some(2)),
        // Exercises multiplication that would overflow u64 without widening.
        (u64::MAX - 1, u64::MAX, Some(2)),
        (0, 0, None),
        (1, 0, None),
        (1, 1, None),
        (0, 7, None),
        (6, 9, None),
        (10, 20, None),
    ] {
        assert_eq!(n_order(a, n), expected, "a = {a}, n = {n}");
    }
}

#[test]
fn test_n_order_matches_small_power_cycles() {
    for n in 2..=50_u64 {
        for a in 0..=2 * n {
            // Bound the search independently, even when no order exists.
            // Values are small enough that multiplication cannot overflow.
            let mut residue = 1;
            let expected = (1..n).find(|_| {
                residue = residue * a % n;
                residue == 1
            });
            assert_eq!(n_order(a, n), expected, "a = {a}, n = {n}");
        }
    }
}
