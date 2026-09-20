use std::ops::{Add, Mul};

pub struct BigInt {
    digits: Vec<u8>,
}

impl BigInt {
    pub fn new(mut n: usize) -> Self {
        let mut digits = vec![];
        while n > 0 {
            digits.push((n % 10) as u8);
            n /= 10;
        }
        Self {
            digits: normalize(digits),
        }
    }

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

impl Add<BigInt> for BigInt {
    type Output = BigInt;

    fn add(self, rhs: BigInt) -> BigInt {
        &self + &rhs
    }
}

impl Add<BigInt> for &BigInt {
    type Output = BigInt;

    fn add(self, rhs: BigInt) -> BigInt {
        self + &rhs
    }
}

impl Add<&BigInt> for BigInt {
    type Output = BigInt;

    fn add(self, rhs: &BigInt) -> BigInt {
        &self + rhs
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
impl Mul<BigInt> for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: BigInt) -> BigInt {
        &self * &rhs
    }
}

impl Mul<BigInt> for &BigInt {
    type Output = BigInt;

    fn mul(self, rhs: BigInt) -> BigInt {
        self * &rhs
    }
}

impl Mul<&BigInt> for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: &BigInt) -> BigInt {
        &self * rhs
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
