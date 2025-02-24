extern crate rand;
use rand::Rng;

pub struct Shamir {
    threshold: usize,
    shares: usize,
    prime: u64,
}

impl Shamir {
    pub fn new(threshold: usize, shares: usize, prime: u64) -> Self {
        assert!(threshold <= shares, "Threshold cannot be greater than the shares");
        Self { threshold, shares, prime }
    }

    pub fn split_secret(&self, secret: u64) -> Vec<(u64, u64)> {
        let mut rng = rand::thread_rng();
        let mut coeffs = vec![secret];
        for _ in 1..self.threshold {
            coeffs.push(rng.gen_range(1..self.prime));
        }

        let mut shares = Vec::new();

        for x in 1..=self.shares as u64 {
            let mut y = 0;
            for (i, &coeff) in coeffs.iter().enumerate() {
                y = (y + coeff * mod_exp(x, i as u64, self.prime)) % self.prime;
            }
            shares.push((x, y));
        }
        shares
    }

    pub fn reconstruct_secret(&self, shares: &[(u64, u64)]) -> u64 {
        assert!(shares.len() >= self.threshold, "Not enough shares to reconstruct the secret");

        let mut secret = 0;

        for (i, &(xi, yi)) in shares.iter().enumerate() {
            let mut num = 1;
            let mut denom = 1;

            for (j, &(xj, _)) in shares.iter().enumerate() {
                if i != j {
                    num = (num * ((self.prime + xj) % self.prime)) % self.prime;
                    denom = (denom * ((self.prime + xi - xj) % self.prime)) % self.prime;
                }
            }

            let denom_inv = mod_inverse(denom, self.prime);
            let lagrange_coeff = (num * denom_inv) % self.prime;
            secret = (secret + (yi * lagrange_coeff) % self.prime) % self.prime;
        }

        secret
    }
}

// Modular exponentiation (x^y % prime)
fn mod_exp(mut base: u64, mut exp: u64, prime: u64) -> u64 {
    let mut result = 1;
    base %= prime;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % prime;
        }
        base = (base * base) % prime;
        exp /= 2;
    }
    result
}

// Fixed modular inverse function (now using i64 to prevent underflow)
fn mod_inverse(a: u64, prime: u64) -> u64 {
    let (mut old_r, mut r) = (prime as i64, a as i64);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_t, t) = (t, old_t - quotient * t);
    }

    if old_r > 1 {
        panic!("{} has no modular inverse modulo {}", a, prime);
    }

    ((old_t % prime as i64 + prime as i64) % prime as i64) as u64
}
