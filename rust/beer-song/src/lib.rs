pub fn verse(n: u32) -> String {
	match n {
		3..=99 => {
			format!(
				"{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it \
				 around, {} bottles of beer on the wall.",
				n,
				n,
				n - 1
			)
		}
		2 => {
			"2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, \
			 1 bottle of beer on the wall."
				.to_string()
		}
		1 => {
			"1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no \
			 more bottles of beer on the wall."
				.to_string()
		}
		0 => {
			"No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and \
			 buy some more, 99 bottles of beer on the wall."
				.to_string()
		}
		_ => String::new(),
	}
}

pub fn sing(start: u32, end: u32) -> String {
	todo!("sing verses {start} to {end}, inclusive")
}
