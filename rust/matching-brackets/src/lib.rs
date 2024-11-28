use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
	// Map key is right bracket, and map value is corresponding left bracket.
	let brackets_map = HashMap::from([('}', '{'), (']', '['), (')', '(')]);

	// Make use of the FILO feature of stack, the last element is top of the stack.
	let mut stack: Vec<char> = Vec::new();

	for c in string.chars() {
		// if c is left bracket, then just stack.push(c).
		if brackets_map
			.values()
			.any(|&k| k == c)
		{
			stack.push(c);
		}

		// if c is right bracket,
		//   if stack is not empty, and top of stack is corresponding left bracket of c,
		//     stack.pop()
		//   else
		//     return false
		if brackets_map
			.keys()
			.any(|&v| v == c)
		{
			if !stack.is_empty() && stack.last().copied().unwrap() == brackets_map[&c] {
				stack.pop();
			} else {
				return false;
			}
		}
	}

	stack.is_empty()
}
