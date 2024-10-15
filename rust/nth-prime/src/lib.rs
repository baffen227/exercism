// https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes

use lazy_static::lazy_static;

lazy_static! {
	static ref primes: Vec<u32> = {
		const MAX_SIZE: usize = 1000005;
		let mut is_prime = [true; MAX_SIZE];

		let mut p = 2;
		while p * p < MAX_SIZE {
			if is_prime[p] {
				let mut i = p * p;
				while i < MAX_SIZE {
					is_prime[i] = false;
					i += p;
				}
			}
			p += 1;
		}

		let mut v: Vec<u32> = vec![];
		p = 2;
		while p < MAX_SIZE {
			if is_prime[p] {
				v.push(p as u32);
			}
			p += 1;
		}
		v
	};
}

pub fn nth(n: u32) -> u32 {
	*(primes
		.get(n as usize)
		.unwrap())
}
