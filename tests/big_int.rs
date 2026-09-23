use pe_rs::big_int::BigInt;

mod construction {
    use pe_rs::big_int::BigInt;

    #[test]
    fn from_valid_str() {
        let cases = [
            ("123", &vec![3, 2, 1]),
            ("0", &vec![0]),
            ("000", &vec![0]),
            ("00123", &vec![3, 2, 1]),
            (
                "123456789012345678901234567890",
                &vec![
                    0, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 9, 8, 7, 6, 5,
                    4, 3, 2, 1,
                ],
            ),
        ];

        for (s, expect_digit) in cases {
            assert_eq!(BigInt::from(s).digits(), expect_digit);
        }
    }

    #[test]
    fn from_invalid_str() {
        let invalid_cases = [
            "", "-123", "+123", "1.23", "12a3", " 123", "123 ", "1 23", "1 2 3",
        ];
        for s in invalid_cases {
            assert!(
                std::panic::catch_unwind(|| BigInt::from(s)).is_err(),
                "expected a panic for input {s:?}"
            );
        }
    }

    #[test]
    fn from_valid_num() {
        let cases = [(0, "0"), (1, "1"), (123, "123"), (i32::MAX, "2147483647")];

        for (n, expected) in cases {
            assert_eq!(BigInt::from(n).to_string(), expected);
        }
    }

    #[test]
    fn from_invalid_num() {
        for n in [-1, -123, i32::MIN] {
            assert!(
                std::panic::catch_unwind(|| BigInt::from(n)).is_err(),
                "expected a panic for {n}"
            );
        }
    }
}

#[test]
fn addition() {
    let sum = BigInt::from(999) + BigInt::from(1);
    assert_eq!(sum.to_string(), "1000");
}

#[test]
fn multiplication() {
    let n = BigInt::from(99);
    assert_eq!((&n * &n).to_string(), "9801");
    assert_eq!((&n * BigInt::from(0)).to_string(), "0");
}

mod comparison {
    use super::*;

    fn pow2(exp: usize) -> BigInt {
        let mut n = BigInt::from(1);
        let two = BigInt::from(2);
        for _ in 0..exp {
            n = n * &two;
        }
        n
    }

    #[test]
    fn basic() {
        assert_eq!(BigInt::from(1), BigInt::from(1));

        for (smaller, larger) in [(1, 2), (9, 10), (19, 20), (109, 110)] {
            let a = BigInt::from(smaller);
            let b = BigInt::from(larger);

            assert!(a < b);
            assert!(b > a);
            assert_ne!(a, b);
        }
    }

    #[test]
    fn large_number() {
        assert_eq!(pow2(1000), pow2(1000));
        assert_ne!(pow2(999), pow2(1000));
        assert!(pow2(999) < pow2(1000));
    }
}

mod subtraction {
    use pe_rs::big_int::BigInt;

    #[test]
    fn basic() {
        let cases = [
            (0, 0, 0),
            (7, 0, 7),
            (123, 123, 0),
            (9, 4, 5),
            (543, 321, 222),
            (10, 1, 9),
            (1_000, 1, 999),
            (1_002, 9, 993),
            (1_010, 11, 999),
            (1_234, 234, 1_000),
            (1_005, 1_000, 5),
            (1_000, 999, 1),
            (12_345, 6_789, 5_556),
        ];

        for (a, b, res) in cases {
            assert_eq!(BigInt::from(a) - BigInt::from(b), BigInt::from(res));
        }
    }
}

mod div_rem {
    use pe_rs::big_int::BigInt;

    #[test]
    fn basic() {
        let cases = [
            (0, 1, 0, 0),
            (0, 123, 0, 0),
            (1, 1, 1, 0),
            (1, 2, 0, 1),
            (9, 2, 4, 1),
            (10, 2, 5, 0),
            (123, 123, 1, 0),
            (999, 1_000, 0, 999),
            (1_000, 999, 1, 1),
            (1_000, 10, 100, 0),
            (1_001, 10, 100, 1),
            (1_005, 5, 201, 0),
            (1_234, 12, 102, 10),
            (10_000, 101, 99, 1),
            (12_345, 67, 184, 17),
            (999_999, 1, 999_999, 0),
            (999_999, 9, 111_111, 0),
            (1_002_003, 1_000, 1_002, 3),
            (987_654_321, 12_345, 80_004, 4_941),
        ];

        for (dividend, divisor, quotient, remainder) in cases {
            let (q, r) = BigInt::from(dividend).div_rem(&BigInt::from(divisor));
            assert_eq!(q, BigInt::from(quotient));
            assert_eq!(r, BigInt::from(remainder));
        }
    }
}
