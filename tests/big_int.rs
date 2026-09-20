use pe_rs::big_int::BigInt;

#[test]
fn construction() {
    let zero = BigInt::new(0);
    assert_eq!(zero.digits(), &[0]);
    assert_eq!(zero.to_string(), "0");

    let n = BigInt::new(123);
    assert_eq!(n.digits(), &[3, 2, 1]);
    assert_eq!(n.digit_count(), 3);
    assert_eq!(n.digits_sum(), 6);
    assert_eq!(n.to_u128(), 123);
    assert_eq!(n.to_string(), "123");
}

#[test]
fn addition() {
    let sum = BigInt::new(999) + BigInt::new(1);
    assert_eq!(sum.to_string(), "1000");
}

#[test]
fn multiplication() {
    let n = BigInt::new(99);
    assert_eq!((&n * &n).to_string(), "9801");
    assert_eq!((&n * BigInt::new(0)).to_string(), "0");
}

mod comparison {
    use super::*;

    fn pow2(exp: usize) -> BigInt {
        let mut n = BigInt::new(1);
        let two = BigInt::new(2);
        for _ in 0..exp {
            n = n * &two;
        }
        n
    }

    #[test]
    fn basic() {
        assert_eq!(BigInt::new(1), BigInt::new(1));

        for (smaller, larger) in [(1, 2), (9, 10), (19, 20), (109, 110)] {
            let a = BigInt::new(smaller);
            let b = BigInt::new(larger);

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
            assert_eq!(BigInt::new(a) - BigInt::new(b), BigInt::new(res));
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
            let (q, r) = BigInt::new(dividend).div_rem(&BigInt::new(divisor));
            assert_eq!(q, BigInt::new(quotient));
            assert_eq!(r, BigInt::new(remainder));
        }
    }
}
