pub fn build_proverb(list: &[&str]) -> String {
	let mut result = vec![];
	let mut iter = list.iter().peekable();
	while let Some(first) = iter.next() {
		match iter.peek() {
			Some(second) => {
				let s = format!("For want of a {} the {} was lost.", first, second);
				result.push(s);
			}
			None => {
				let s = format!("And all for the want of a {}.", list[0]);
				result.push(s);
			}
		}
	}
	result.join("\n")
}
