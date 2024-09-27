pub fn annotate(minefield: &[&str]) -> Vec<String> {
	use std::collections::HashMap;
	type FieldMap = HashMap<SquareCoordinate, char>;

	#[derive(Eq, Hash, PartialEq, Clone)]
	struct SquareCoordinate {
		row_idx: i8,
		col_idx: i8,
	}

	struct MinefieldMap {
		row_num: i8,
		col_num: i8,
		field_map: FieldMap,
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
				let mut field_map = FieldMap::new();
				for (idx, c) in minefield
					.iter()
					.flat_map(|s| s.chars())
					.enumerate()
				{
					let i = (idx / column_num) as i8;
					let j = (idx % column_num) as i8;
					field_map.insert(
						SquareCoordinate {
							row_idx: i,
							col_idx: j,
						},
						c,
					);
				}
				RawMinefieldStatus::Valid(MinefieldMap {
					row_num: row_num as i8,
					col_num: column_num as i8,
					field_map,
				})
			}
		}
	}

	impl MinefieldMap {
		fn do_annotate(self) -> Vec<String> {
			let mut annotated_field_map = self.field_map.clone();

			#[rustfmt::skip]
			let deltas = [
				(-1, -1), (-1, 0), (-1, 1),
				( 0, -1),          ( 0, 1),
				( 1, -1), ( 1, 0), ( 1, 1),
			];

			for (cur_idx, c) in &self.field_map {
				// count the number of mine adjacent to current square
				if *c != '*' {
					let mut cnt = 0_u8;
					for d in deltas {
						let ri = cur_idx.row_idx + d.0;
						let ci = cur_idx.col_idx + d.1;
						if let Some(r) = self
							.field_map
							.get(&SquareCoordinate {
								row_idx: ri,
								col_idx: ci,
							}) {
							if *r == '*' {
								cnt += 1;
							}
						}
					}
					if let Some(c) = annotated_field_map.get_mut(cur_idx) {
						if cnt != 0 {
							*c = char::from_digit(cnt.into(), 10)
								.expect("digit must be in the range 0-9");
						}
					}
				}
			}

			// transform annotated_field_map into Vec<String>
			let mut annotated_res: Vec<String> = vec![];
			for row_idx in 0..self.row_num {
				let mut s = String::new();
				for col_idx in 0..self.col_num {
					if let Some(c) = annotated_field_map.get(&SquareCoordinate { row_idx, col_idx })
					{
						s.push(*c);
					}
				}
				annotated_res.push(s);
			}
			annotated_res
		}
	}

	match verify(minefield) {
		RawMinefieldStatus::NoRows => Vec::new(),
		RawMinefieldStatus::NoColumns => vec!["".to_string()],
		RawMinefieldStatus::Invalid => Vec::new(),
		RawMinefieldStatus::Valid(map) => map.do_annotate(),
	}
}
