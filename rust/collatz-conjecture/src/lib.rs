pub fn collatz(n: u64) -> Option<u64> {
	let mut i: u64 = n;
	let mut cnt: u64 = 0;

	if i == 0 {
		return None;
	}

	while i > 1 {
		if i % 2 == 0 {
			i /= 2;
		} else {
			i = i * 3 + 1;
		}
		cnt += 1;
	}

	Some(cnt)
}
