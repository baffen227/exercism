pub fn brackets_are_balanced(string: &str) -> bool {
	let mut stack: Vec<char> = Vec::new();
	for c in string
		.chars()
		.filter(|c| *c == '{' || *c == '}' || *c == '[' || *c == ']' || *c == '(' || *c == ')')
	{
		if c == '{' || c == '[' || c == '(' {
			stack.push(c);
		}
		if (c == '}' || c == ']' || c == ')') && stack.is_empty() {
			return false;
		}
		if c == '}' {
			// if stack is not empty,
			//     if stack.last is not '{', then return false.
			//     if stack.last is '{', then stack.pop().
			if stack.last().copied().unwrap() == '{' {
				stack.pop();
			} else {
				return false;
			}
		}
		if c == ']' {
			// if stack is not empty,
			//     if stack.last is not '[', then return false.
			//     if stack.last is '[', then stack.pop().
			if stack.last().copied().unwrap() == '[' {
				stack.pop();
			} else {
				return false;
			}
		}
		if c == ')' {
			// if stack is not empty,
			//     if stack.last is not '(', then return false.
			//     if stack.last is '(', then stack.pop().
			if stack.last().copied().unwrap() == '(' {
				stack.pop();
			} else {
				return false;
			}
		}
	}
	stack.is_empty()
}
