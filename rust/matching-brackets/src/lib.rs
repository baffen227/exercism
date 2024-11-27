pub fn brackets_are_balanced(string: &str) -> bool {
	let mut stack: Vec<char> = Vec::new();
	for c in string
		.chars()
		.filter(|c| *c == '{' || *c == '}' || *c == '[' || *c == ']' || *c == '(' || *c == ')')
	{
		if c == '{' || c == '[' || c == '(' {
			stack.push(c);
		}
		if c == '}' {
			// if stack is empty, return false
			// if stack is not empty, check if stack.last is '{',
			//
		}
	}
	stack.is_empty()
}
