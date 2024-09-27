pub fn annotate(minefield: &[&str]) -> Vec<String> {
	use std::collections::HashMap;
	struct MinefieldMap {
		row_num: u8,
		column_num: u8,
		field_map: HashMap<(u8, u8), char>,
	}

	enum RawMinefieldStatus {
		NoRows,
		NoColumns,
		Invalid,
		Valid(MinefieldMap),
	}

	fn verify(minefield: &[&str]) -> RawMinefieldStatus {
		let row_num = minefield.len();
		if row_num == 0 {
			RawMinefieldStatus::NoRows
		} else if minefield[0].is_empty() {
			RawMinefieldStatus::NoColumns
		} else {
			let column_num = minefield[0].len();
			if minefield
				.iter()
				.any(|s| s.len() != column_num)
			{
				RawMinefieldStatus::Invalid
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
				RawMinefieldStatus::Valid(MinefieldMap {
					row_num: row_num as u8,
					column_num: column_num as u8,
					field_map,
				})
			}
		}
	}

	impl MinefieldMap {
		fn do_annotate(&mut self) -> Vec<String> {
			let mut annotated_map = HashMap::<(u8, u8), char>::new();
		}
	}

	match verify(minefield) {
		RawMinefieldStatus::NoRows => Vec::new(),
		RawMinefieldStatus::NoColumns => vec!["".to_string()],
		RawMinefieldStatus::Invalid => Vec::new(),
		RawMinefieldStatus::Valid(map) => map.do_annotate(),
	}
}
