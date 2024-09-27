pub fn annotate(minefield: &[&str]) -> Vec<String> {
	struct MindfieldDim {
		row_num: u8,
		column_num: u8,
	}

	enum InitialStatus {
		NoRows,
		NoColumns,
		Invalid,
		Valid(MindfieldDim),
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
				InitialStatus::Valid(MindfieldDim {
					row_num: row_num as u8,
					column_num: column_num as u8,
				})
			}
		}
	}

	fn do_annotate(minefield: &[&str], dim: MindfieldDim) -> Vec<String> {
		minefield.iter().flatten()
	}

	match verify(minefield) {
		InitialStatus::NoRows => Vec::new(),
		InitialStatus::NoColumns => vec!["".to_string()],
		InitialStatus::Invalid => Vec::new(),
		InitialStatus::Valid(dim) => do_annotate(minefield, dim),
	}
}
