use crate::big_int::BigInt;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
pub struct PeriodicCF {
    pub integer_part: usize,
    pub period: Vec<usize>,
}

impl fmt::Display for PeriodicCF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}", self.integer_part)?;

        if !self.period.is_empty() {
            write!(f, "; (")?;
            for (i, term) in self.period.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{term}")?;
            }
            write!(f, ")")?;
        }

        write!(f, "]")
    }
}

impl PeriodicCF {
    pub fn period_len(&self) -> usize {
        self.period.len()
    }

    /// Return the zero-based `idx`th convergent as (numerator, denominator)
    pub fn convergent(&self, idx: usize) -> (BigInt, BigInt) {
        if idx == 0 || self.period.is_empty() {
            return (BigInt::new(self.integer_part), BigInt::new(1));
        }

        let last = self.period[(idx - 1) % self.period_len()];
        let mut num = BigInt::new(last);
        let mut den = BigInt::new(1);

        for i in (0..idx - 1).rev() {
            let a = BigInt::new(self.period[i % self.period_len()]);
            let new_num = a * &num + &den;
            den = num;
            num = new_num;
        }

        let a = BigInt::new(self.integer_part);
        let new_num = a * &num + &den;
        (new_num, num)
    }
}

/// Get the continued fraction of square root of n
pub fn period(n: usize) -> PeriodicCF {
    let mut p = 0;
    let mut q = 1;
    let mut period = vec![];
    let mut seen = HashSet::new();

    loop {
        let a = (((n as f64).sqrt() + p as f64) / q as f64).floor() as usize;
        if !seen.insert((p, q)) || q == 0 {
            break;
        }
        period.push(a);
        p = a * q - p;
        q = (n - p * p) / q;
    }

    let integer_part = period.remove(0);
    PeriodicCF {
        integer_part,
        period,
    }
}
