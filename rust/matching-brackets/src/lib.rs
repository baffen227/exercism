use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
	let brackets_map = HashMap::from([('{', '}'), ('[', ']'), ('(', ')')]);
	let mut stack: Vec<char> = Vec::new();
	for c in string.chars() {
		// if c is left bracket, then stack.push(c).
		if brackets_map
			.keys()
			.any(|&k| k == c)
		{
			stack.push(c);
		}
		// if c is right bracket and stack is empty, then return false.
		if brackets_map
			.values()
			.any(|&v| v == c)
		{
			if stack.is_empty() {
				return false;
			} else {
				// TODO
				// if stack.last().copied().unwrap() == brackets {
				// stack.pop();
				// } else {
				// return false;
				// }
			}
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
