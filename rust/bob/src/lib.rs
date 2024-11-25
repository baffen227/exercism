pub fn reply(message: &str) -> &str {
	let message = message.trim();

	// from most specific to general cases

	if message.is_empty()
		|| message
			.chars()
			.all(|c| c.is_ascii_whitespace())
	{
		// message is empty or only consists of whitespace characters.
		"Fine. Be that way!"
	} else if message
		.chars()
		.filter(|c| (*c).is_alphabetic())
		.count()
		== 0
	{
		// there isn't any alphabetic character in the message.
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
		// there are alphabetic characters in the message, and they are all uppercase.
		if message.ends_with("?") {
			"Calm down, I know what I'm doing!"
		} else {
			"Whoa, chill out!"
		}
	} else if message.ends_with("?") {
		// there are alphabetic characters, but not all of them are uppercase.
		"Sure."
	} else {
		"Whatever."
	}
}
