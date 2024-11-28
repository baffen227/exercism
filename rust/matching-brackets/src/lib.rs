use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
	// HashMap keys are right brackets, and values are corresponding left brackets.
	let brackets_map = HashMap::from([('}', '{'), (']', '['), (')', '(')]);

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
		//   if stack is not empty, and the top is the corresponding left bracket of c,
		//     stack.pop()
		//   else
		//     return false
		if brackets_map
			.keys()
			.any(|&v| v == c)
		{
			if !stack.is_empty() && (stack.last() == brackets_map.get(&c)) {
				stack.pop();
			} else {
				return false;
			}
		}
	}

	stack.is_empty()
}
