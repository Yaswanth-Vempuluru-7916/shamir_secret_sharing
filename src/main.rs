use rand::Rng;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::HashMap;

/// Generate random polynomial coefficients with secret as the constant term.
fn generate_coefficients(secret: BigUint, threshold: usize, prime: &BigUint) -> Vec<BigUint> {
    let mut rng = rand::thread_rng();
    let mut coeffs = vec![secret];
    for _ in 1..threshold {
        let coeff = BigUint::from(rng.gen::<u64>()) % prime;
        coeffs.push(coeff);
    }
    coeffs
}

/// Evaluate polynomial at x using Horner's method.
fn evaluate_polynomial(coeffs: &[BigUint], x: &BigUint, prime: &BigUint) -> BigUint {
    let mut result = BigUint::zero();
    for coeff in coeffs.iter().rev() {
        result = (result * x + coeff) % prime;
    }
    result
}

/// Generate commitments using modular arithmetic.
pub fn generate_commitments(coeffs: &[BigUint], prime: &BigUint) -> Vec<BigUint> {
    coeffs.iter().map(|c| c % prime).collect()
}

/// Verify a share against the commitments.
pub fn verify_share(x: usize, share: &BigUint, commitments: &[BigUint], prime: &BigUint) -> bool {
    let x_big = BigUint::from(x);
    let mut expected_value = BigUint::zero();
    
    for (i, commitment) in commitments.iter().enumerate() {
        expected_value = (expected_value + commitment * x_big.modpow(&BigUint::from(i), prime)) % prime;
    }
    expected_value == *share
}

/// Generate secret shares with commitments.
pub fn generate_shares(secret: BigUint, threshold: usize, num_shares: usize, prime: BigUint) -> (HashMap<usize, BigUint>, Vec<BigUint>) {
    let coeffs = generate_coefficients(secret, threshold, &prime);
    let commitments = generate_commitments(&coeffs, &prime);
    let shares = (1..=num_shares).map(|x| {
        let x_big = BigUint::from(x);
        (x, evaluate_polynomial(&coeffs, &x_big, &prime))
    }).collect();
    (shares, commitments)
}

/// Lagrange interpolation to reconstruct the secret.
pub fn reconstruct_secret(shares: &HashMap<usize, BigUint>, prime: &BigUint) -> BigUint {
    let mut secret = BigUint::zero();
    for (&i, y_i) in shares.iter() {
        let mut num = BigUint::one();
        let mut denom = BigUint::one();
        for (&j, _) in shares.iter() {
            if i != j {
                let diff = (BigUint::from(j) + prime - BigUint::from(i)) % prime;
                num = (num * BigUint::from(j)) % prime;
                denom = (denom * diff) % prime;
            }
        }
        let lagrange_coeff = (num * denom.modpow(&(prime - BigUint::from(2u32)), prime)) % prime;
        secret = (secret + y_i * lagrange_coeff) % prime;
    }
    secret
}

fn main() {
    let secret = BigUint::from(123456789u64);
    let prime = BigUint::from(32416190071u64);
    let threshold = 3;
    let num_shares = 5;
    
    let (shares, commitments) = generate_shares(secret.clone(), threshold, num_shares, prime.clone());
    println!("Generated Shares: {:?}", shares);
    println!("Commitments: {:?}", commitments);
    
    for (x, share) in &shares {
        let is_valid = verify_share(*x, share, &commitments, &prime);
        println!("Share {} verification: {}", x, is_valid);
    }
    
    let subset_shares: HashMap<usize, BigUint> = shares.into_iter().take(threshold).collect();
    let reconstructed_secret = reconstruct_secret(&subset_shares, &prime);
    println!("Reconstructed Secret: {}", reconstructed_secret);
}