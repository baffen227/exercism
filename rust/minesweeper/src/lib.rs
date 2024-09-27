pub fn annotate(minefield: &[&str]) -> Vec<String> {
	match verify(minefield) {
		RawMinefieldStatus::NoRows => Vec::new(),
		RawMinefieldStatus::NoColumns => vec!["".to_string()],
		RawMinefieldStatus::Invalid => Vec::new(),
		RawMinefieldStatus::Valid(map) => map.do_annotate(),
	}
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
		let col_num = minefield[0].len();
		let all_row_len_are_the_same = minefield
			.iter()
			.all(|s| s.len() == col_num);
		if all_row_len_are_the_same {
			RawMinefieldStatus::Valid(MinefieldMap::new(minefield))
		} else {
			RawMinefieldStatus::Invalid
		}
	}
}

use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Clone)]
struct SquareCoordinate {
	row_idx: i8,
	col_idx: i8,
}

type FieldMap = HashMap<SquareCoordinate, char>;
struct MinefieldMap {
	row_num: i8,
	col_num: i8,
	field_map: FieldMap,
}

impl MinefieldMap {
	fn new(minefield: &[&str]) -> Self {
		let row_num = minefield.len();
		let col_num = minefield[0].len();

		let mut field_map = FieldMap::new();
		minefield
			.iter()
			.flat_map(|s| s.chars())
			.enumerate()
			.for_each(|(idx, c)| {
				field_map.insert(
					SquareCoordinate {
						row_idx: (idx / col_num) as i8,
						col_idx: (idx % col_num) as i8,
					},
					c,
				);
			});

		Self {
			row_num: row_num as i8,
			col_num: col_num as i8,
			field_map,
		}
	}

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
				if let Some(c) = annotated_field_map.get(&SquareCoordinate { row_idx, col_idx }) {
					s.push(*c);
				}
			}
			annotated_res.push(s);
		}
		annotated_res
	}
}
