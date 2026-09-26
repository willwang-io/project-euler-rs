use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BigInt {
    digits: Vec<u8>,
}

impl From<&str> for BigInt {
    fn from(s: &str) -> Self {
        assert!(
            !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()),
            "expected a nonempty string of decimal digits"
        );
        Self {
            digits: normalize(s.bytes().rev().map(|b| b - b'0').collect()),
        }
    }
}

impl From<usize> for BigInt {
    fn from(mut n: usize) -> Self {
        let mut digits = vec![];
        while n > 0 {
            digits.push((n % 10) as u8);
            n /= 10;
        }
        Self {
            digits: normalize(digits),
        }
    }
}

impl From<i32> for BigInt {
    fn from(n: i32) -> Self {
        let n = usize::try_from(n).expect("BigInt requires a nonnegative integer");
        Self::from(n)
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.digit_count()
            .cmp(&other.digit_count())
            .then_with(|| self.digits.iter().rev().cmp(other.digits.iter().rev()))
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for d in self.digits().iter().rev() {
            write!(f, "{d}")?;
        }
        Ok(())
    }
}

impl BigInt {
    pub fn digit_count(&self) -> usize {
        self.digits.len()
    }

    pub fn digits(&self) -> &[u8] {
        &self.digits
    }

    pub fn digits_sum(&self) -> i32 {
        self.digits.iter().map(|&d| i32::from(d)).sum()
    }

    pub fn to_u128(&self) -> u128 {
        self.digits
            .iter()
            .rev()
            .fold(0, |acc, &digit| acc * 10 + u128::from(digit))
    }

    pub fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        assert_ne!(rhs.digits, [0], "division by zero");

        let mut quotient = vec![0; self.digit_count()];
        let mut rem = Self::from(0);

        for (i, &d) in self.digits.iter().enumerate().rev() {
            if rem.digits == [0] {
                rem.digits[0] = d;
            } else {
                rem.digits.insert(0, d);
            }
            while &rem >= rhs {
                rem = rem - rhs;
                quotient[i] += 1;
            }
        }

        (
            Self {
                digits: normalize(quotient),
            },
            rem,
        )
    }
}

impl Add<&BigInt> for &BigInt {
    type Output = BigInt;

    fn add(self, rhs: &BigInt) -> Self::Output {
        let len = self.digit_count().max(rhs.digit_count());
        let mut ans = Vec::with_capacity(len + 1);
        let mut carry = 0;

        for i in 0..len {
            let x = *self.digits().get(i).unwrap_or(&0);
            let y = *rhs.digits().get(i).unwrap_or(&0);
            let total = x + y + carry;

            ans.push(total % 10);
            carry = total / 10;
        }

        if carry != 0 {
            ans.push(carry);
        }

        BigInt {
            digits: normalize(ans),
        }
    }
}

impl Mul<&BigInt> for &BigInt {
    type Output = BigInt;

    fn mul(self, rhs: &BigInt) -> BigInt {
        let mut ans = vec![0; self.digit_count() + rhs.digit_count()];

        for (i, &x) in self.digits().iter().enumerate() {
            let mut carry = 0;

            for (j, &y) in rhs.digits().iter().enumerate() {
                let total = ans[i + j] + x * y + carry;
                ans[i + j] = total % 10;
                carry = total / 10;
            }

            ans[i + rhs.digit_count()] = carry;
        }

        BigInt {
            digits: normalize(ans),
        }
    }
}

impl Sub<&BigInt> for &BigInt {
    type Output = BigInt;

    fn sub(self, rhs: &BigInt) -> BigInt {
        assert!(self >= rhs, "subtraction requires a >= b");

        let mut ans = Vec::with_capacity(self.digit_count());
        let mut borrow = 0;

        for (i, &x) in self.digits().iter().enumerate() {
            let y = rhs.digits().get(i).copied().unwrap_or(0) + borrow;

            if x >= y {
                ans.push(x - y);
                borrow = 0;
            } else {
                ans.push(x + 10 - y);
                borrow = 1;
            }
        }

        BigInt {
            digits: normalize(ans),
        }
    }
}

impl Div<&BigInt> for &BigInt {
    type Output = BigInt;

    fn div(self, rhs: &BigInt) -> BigInt {
        self.div_rem(rhs).0
    }
}

impl Rem<&BigInt> for &BigInt {
    type Output = BigInt;

    fn rem(self, rhs: &BigInt) -> BigInt {
        self.div_rem(rhs).1
    }
}

fn normalize(mut d: Vec<u8>) -> Vec<u8> {
    while d.len() > 1 && d.last() == Some(&0) {
        d.pop();
    }
    if d.is_empty() {
        d.push(0);
    }
    d
}

macro_rules! impl_owned_binop {
      ($trait:ident, $method:ident, $op:tt) => {
          impl std::ops::$trait<BigInt> for BigInt {
              type Output = BigInt;

              fn $method(self, rhs: BigInt) -> BigInt {
                  &self $op &rhs
              }
          }

          impl std::ops::$trait<BigInt> for &BigInt {
              type Output = BigInt;

              fn $method(self, rhs: BigInt) -> BigInt {
                  self $op &rhs
              }
          }

          impl std::ops::$trait<&BigInt> for BigInt {
              type Output = BigInt;

              fn $method(self, rhs: &BigInt) -> BigInt {
                  &self $op rhs
              }
          }
      };
  }

impl_owned_binop!(Add, add, +);
impl_owned_binop!(Sub, sub, -);
impl_owned_binop!(Mul, mul, *);
impl_owned_binop!(Div, div, /);
impl_owned_binop!(Rem, rem, %);
