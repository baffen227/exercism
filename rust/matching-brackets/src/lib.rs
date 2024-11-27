pub fn brackets_are_balanced(string: &str) -> bool {
	let mut stack: Vec<char> = Vec::new();
	for c in string.chars() {
		if "{[(".contains(c) {
			stack.push(c);
		}
		if "}])".contains(c) && stack.is_empty() {
			return false;
		}
		if c == '}' {
			if stack.last().copied().unwrap() == '{' {
				stack.pop();
			} else {
				return false;
			}
		}
		if c == ']' {
			if stack.last().copied().unwrap() == '[' {
				stack.pop();
			} else {
				return false;
			}
		}
		if c == ')' {
			if stack.last().copied().unwrap() == '(' {
				stack.pop();
			} else {
				return false;
			}
		}
	}
	stack.is_empty()
}
