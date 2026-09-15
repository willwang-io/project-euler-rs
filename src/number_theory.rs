use std::collections::BTreeMap;

#[inline]
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn gaussian_gcd(mut a: GaussianInt, mut b: GaussianInt) -> GaussianInt {
    while b.norm() != 0 {
        let t = b;
        b = a.rem(b);
        a = t;
    }
    a
}

pub fn sig(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut total = 0;
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in 1..=sqrt_n {
        if n % i == 0 {
            total += i;
            if i * i != n {
                total += n / i;
            }
        }
    }
    total
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn mod_pow(mut base: u64, mut exp: u64, modules: u64) -> u64 {
    let mut ans = 1;
    base %= modules;

    while exp > 0 {
        if exp % 2 == 1 {
            ans = mod_mul(ans, base, modules);
        }
        base = mod_mul(base, base, modules);
        exp /= 2;
    }
    ans
}

pub fn sieve_of_eratosthenes(limit: usize) -> Vec<bool> {
    if limit < 2 {
        vec![false; limit + 1]
    } else {
        let mut is_prime = vec![true; limit + 1];
        is_prime[0] = false;
        is_prime[1] = false;

        let limit_sqrt = (limit as f64).sqrt() as usize;
        for i in 2..=limit_sqrt {
            if is_prime[i] {
                for j in (i * i..=limit).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }
        is_prime
    }
}

pub fn smallest_prime_factor(limit: usize) -> Vec<usize> {
    let mut spf: Vec<usize> = (0..=limit).collect();
    let limit_sqrt = (limit as f64).sqrt() as usize;

    for i in 2..=limit_sqrt {
        if i == spf[i] {
            for j in (i * i..=limit).step_by(i) {
                if spf[j] == j {
                    spf[j] = i;
                }
            }
        }
    }
    spf
}

pub fn is_prime(n: u64) -> bool {
    if n == 2 || n == 3 {
        true
    } else if n < 2 || n.is_multiple_of(2) {
        false
    } else {
        let s = (n - 1).trailing_zeros();
        let d = (n - 1) >> s;
        let bases = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
        'witness_loop: for &a in &bases {
            if a % n == 0 {
                continue;
            }

            let mut x = mod_pow(a, d, n);
            if x == 1 || x == n - 1 {
                continue 'witness_loop;
            }

            for _ in 1..s {
                x = mod_mul(x, x, n);
                if x == n - 1 {
                    continue 'witness_loop;
                }
            }
            return false;
        }
        true
    }
}

pub fn prime_factors(mut n: u64) -> Vec<(u64, u32)> {
    if n == 0 || n == 1 {
        vec![]
    } else {
        let mut factors_map = BTreeMap::new();
        let trailing_zeros = n.trailing_zeros();
        if trailing_zeros > 0 {
            factors_map.insert(2, trailing_zeros);
            n >>= trailing_zeros;
        }
        factor_recursive(n, &mut factors_map);
        factors_map.into_iter().collect()
    }
}

pub fn divisors(n: u64) -> Vec<u64> {
    let mut d = vec![];
    for x in 1..=n.isqrt() {
        if n.is_multiple_of(x) {
            let y = n / x;
            d.push(x);
            if x != y {
                d.push(y);
            }
        }
    }
    d.sort();
    d
}

pub fn euler_totient(mut n: u64) -> u64 {
    let mut result = n;
    let mut p = 2;

    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            result -= result / p;
        }
        p += 1;
    }

    if n > 1 {
        result -= result / n;
    }

    result
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GaussianInt {
    pub r: i64,
    pub i: i64,
}

impl GaussianInt {
    pub fn new(r: i64, i: i64) -> Self {
        Self { r, i }
    }

    pub fn norm(&self) -> i64 {
        self.r * self.r + self.i * self.i
    }

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.r * other.r - self.i * other.i,
            self.r * other.i + self.i * other.r,
        )
    }

    fn rem(self, other: Self) -> Self {
        let denominator = other.norm();
        if denominator == 0 {
            panic!("Division by zero");
        }
        let real_num = self.r * other.r + self.i * other.i;
        let imag_num = self.i * other.r - self.r * other.i;

        let q_r = (real_num as f64 / denominator as f64).round() as i64;
        let q_i = (imag_num as f64 / denominator as f64).round() as i64;

        let quotient = GaussianInt::new(q_r, q_i);
        let prod = other.mul(quotient);

        GaussianInt::new(self.r - prod.r, self.i - prod.i)
    }
}

// ------------------
// Helpers Methods
// ------------------
fn factor_recursive(n: u64, factors: &mut BTreeMap<u64, u32>) {
    if n == 1 {
        return;
    }
    if is_prime(n) {
        *factors.entry(n).or_insert(0) += 1;
        return;
    }

    if n < 1_000_000 {
        trial_division_map(n, factors);
    } else {
        let divisors = pollard_rho(n);
        factor_recursive(divisors, factors);
        factor_recursive(n / divisors, factors);
    }
}

fn trial_division_map(mut n: u64, factors: &mut BTreeMap<u64, u32>) {
    let mut d = 3;
    while d * d <= n {
        while n.is_multiple_of(d) {
            *factors.entry(d).or_insert(0) += 1;
            n /= d;
        }
        d += 2;
    }
    if n > 1 {
        *factors.entry(n).or_insert(0) += 1;
    }
}

fn pollard_rho(n: u64) -> u64 {
    if n.is_multiple_of(2) {
        return 2;
    }
    let mut c = 1;
    loop {
        let factor = pollard_rho_inner(n, c);
        if factor != n {
            return factor;
        }
        c += 1;
    }
}

fn pollard_rho_inner(n: u64, c: u64) -> u64 {
    let mut x = 2;
    let mut y = 2;
    let mut d = 1;
    let f = |x: u64| ((x as u128 * x as u128) % n as u128 + c as u128) as u64 % n;
    while d == 1 {
        x = f(x);
        y = f(f(y));
        let diff = x.abs_diff(y);
        d = gcd(diff, n);
    }
    d
}

#[inline]
fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}
