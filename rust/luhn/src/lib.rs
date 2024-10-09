/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
	let mut iter1 = code
		.chars()
		.filter(|c| !c.is_whitespace());

	iter1.all(|c| c.is_ascii_digit()) && iter1.count() >= 2
}
