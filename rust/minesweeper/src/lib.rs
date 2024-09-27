pub fn annotate(minefield: &[&str]) -> Vec<String> {
	use std::collections::HashMap;
	struct MinefieldMap {
		row_num: u8,
		column_num: u8,
		field_map: HashMap<(u8, u8), char>,
	}

	enum InitialStatus {
		NoRows,
		NoColumns,
		Invalid,
		Valid(MinefieldMap),
	}

	fn verify(minefield: &[&str]) -> InitialStatus {
		let row_num = minefield.len();
		if row_num == 0 {
			InitialStatus::NoRows
		} else if minefield[0].is_empty() {
			InitialStatus::NoColumns
		} else {
			let column_num = minefield[0].len();
			if minefield
				.iter()
				.any(|s| s.len() != column_num)
			{
				InitialStatus::Invalid
			} else {
				let mut field_map = HashMap::<(u8, u8), char>::new();
				for (idx, c) in minefield
					.iter()
					.flat_map(|s| s.chars())
					.enumerate()
				{
					let i = (idx / column_num) as u8;
					let j = (idx % column_num) as u8;
					field_map.insert((i, j), c);
				}
				InitialStatus::Valid(MinefieldMap {
					row_num: row_num as u8,
					column_num: column_num as u8,
					field_map,
				})
			}
		}
	}

	fn do_annotate(minefield_map: MinefieldMap) -> Vec<String> {}

	match verify(minefield) {
		InitialStatus::NoRows => Vec::new(),
		InitialStatus::NoColumns => vec!["".to_string()],
		InitialStatus::Invalid => Vec::new(),
		InitialStatus::Valid(minefield_map) => do_annotate(minefield_map),
	}
}
