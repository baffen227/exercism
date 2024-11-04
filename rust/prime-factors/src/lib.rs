#![feature(isqrt)]

pub fn factors(n: u64) -> Vec<u64> {
	let mut res = vec![];
	let mut n = n;

	// push the number of 2s that divide n
	while n % 2 == 0 {
		res.push(2);
		n /= 2;
	}

	// n must be odd at this point. So we can skip
	// one element (Note i = i + 2)
	let mut i = 3;
	while i <= n.isqrt() {
		// while i divides n, push i and divide n
		while n % i == 0 {
			res.push(i);
			n /= i;
		}
		i += 2;
	}

	// This condition is to handle the case when n
	// is a prime number greater than 2
	if n > 2 {
		res.push(n);
	}

	res
}
