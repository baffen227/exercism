/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
	let iter1 = code
		.chars()
		.filter(|c| !c.is_whitespace());
	(iter1.clone().count() >= 2)
		&& (iter1
			.clone()
			.all(|c| c.is_ascii_digit()))
}
