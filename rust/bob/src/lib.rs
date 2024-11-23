pub fn reply(message: &str) -> &str {
	let message = message.trim();
	if message.ends_with("?") {
		"Sure."
	} else if message
		.chars()
		.filter(|c| (*c).is_alphanumeric())
		.all(char::is_uppercase)
	{
		"Whoa, chill out!"
	} else {
		"Whatever."
	}
}
