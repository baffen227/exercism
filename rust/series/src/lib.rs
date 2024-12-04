pub fn series(digits: &str, len: usize) -> Vec<String> {
	let mut result = Vec::new();
	let mut start = 0;
	let mut end = len;
	while end <= digits.len() {
		result.push(digits[start..end].to_string());
		start += 1;
		end += 1;
	}
	result
}
