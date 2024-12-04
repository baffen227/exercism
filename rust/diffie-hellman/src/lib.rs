use num_bigint::BigUint;

/// Pick a private key greater than 1 and less than {p}.
pub fn private_key(p: u64) -> u64 {
	p - 1
}

/// Calculate public key using prime numbers {p} and {g}, and private key {a}
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
	BigUint::from(g)
		.modpow(&BigUint::from(a), &BigUint::from(p))
		.to_u64_digits()[0]
}

/// Calculate secret key using prime number {p}, public key {b_pub}, and private
/// key {a}
pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
	BigUint::from(b_pub)
		.modpow(&BigUint::from(a), &BigUint::from(p))
		.to_u64_digits()[0]
}
