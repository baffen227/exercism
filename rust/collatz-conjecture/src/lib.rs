use std::iter::successors;

pub fn collatz(n: u64) -> Option<u64> {
	if n == 0 {
		return None;
	}

	let cnt = successors(Some(n), |i| {
		if *i == 1 {
			None
		} else if *i % 2 == 0 {
			Some(*i / 2)
		} else {
			Some(*i * 3 + 1)
		}
	})
	.count();

	// `cnt` includes number `1`.
	Some((cnt - 1) as u64)
}
