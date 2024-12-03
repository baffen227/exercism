/// Pick a private key greater than 1 and less than {p}.
pub fn private_key(p: u64) -> u64 {
	p - 1
}

/// Calculate public key using prime numbers {p} and {g}, and private key {a}
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
	g.pow(a as u32) % p
}

/// Calculate secret key using prime number {p}, public key {b_pub}, and private
/// key {a}
pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
	b_pub.pow(a as u32) % p
}
