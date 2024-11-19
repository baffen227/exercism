pub fn raindrops(n: u32) -> String {
	if (n % 3 != 0) && (n % 5 != 0) && (n % 7 != 0) {
		n.to_string()
	} else {
		let mut res = vec![];
		if n % 3 == 0 {
			res.push("Pling");
		}
		if n % 5 == 0 {
			res.push("Plang");
		}
		if n % 7 == 0 {
			res.push("Plong");
		}
		res.join("")
	}
}
