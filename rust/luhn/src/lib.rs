/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
	let iter = code
		.chars()
		.filter(|c| !c.is_whitespace())
		.rev();

	(iter.clone().count() >= 2)
		&& (iter
			.clone()
			.all(|c| c.is_ascii_digit()))
		&& (iter
			.map(|c| c.to_digit(10).unwrap())
			.enumerate()
			.map(|(idx, d)| {
				if idx % 2 == 1 {
					if d * 2 > 9 {
						d * 2 - 9
					} else {
						d * 2
					}
				} else {
					d
				}
			})
			.sum::<u32>()
			% 10 == 0)
}
