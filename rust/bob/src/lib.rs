pub fn reply(message: &str) -> &str {
	let message = message.trim();
	if message.is_empty()
		|| message
			.chars()
			.all(|c| c.is_ascii_whitespace())
	{
		"Fine. Be that way!"
	} else if message
		.chars()
		.filter(|c| (*c).is_alphabetic())
		.count()
		== 0
	{
		if message.ends_with("?") {
			"Sure."
		} else {
			"Whatever."
		}
	} else if message
		.chars()
		.filter(|c| (*c).is_alphabetic())
		.all(char::is_uppercase)
	{
		if message.ends_with("?") {
			"Calm down, I know what I'm doing!"
		} else {
			"Whoa, chill out!"
		}
	} else if message.ends_with("?") {
		"Sure."
	} else {
		"Whatever."
	}
}
