use std::collections::HashMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
	// `diagram_vector` is a vector with two string.
	// The first string contains encoded plant characters on the first row.
	// The second string contains encoded plant characters on the second row.
	let mut diagram_vector = Vec::new();
	diagram
		.split('\n')
		.for_each(|row_string| {
			let mut row_vector = Vec::new();
			let mut idx = 0;
			while (idx + 2) <= row_string.len() {
				row_vector.push(&row_string[idx..idx + 2]);
				idx += 2;
			}
			diagram_vector.push(row_vector);
		});

	// key: encoded plant character
	// value: plant full name
	let plant_names = HashMap::from([
		('G', "grass"),
		('C', "clover"),
		('R', "radishes"),
		('V', "violets"),
	]);

	// key: student name
	// value: the index corresponding to the student
	let name2idx = HashMap::from([
		("Alice", 0),
		("Bob", 1),
		("Charlie", 2),
		("David", 3),
		("Eve", 4),
		("Fred", 5),
		("Ginny", 6),
		("Harriet", 7),
		("Ileana", 8),
		("Joseph", 9),
		("Kincaid", 10),
		("Larry", 11),
	]);
	let student_idx = name2idx[student];

	let mut plants_vector = Vec::new();

	diagram_vector[0][student_idx]
		.chars()
		.for_each(|plant| plants_vector.push(plant_names[&plant]));
	diagram_vector[1][student_idx]
		.chars()
		.for_each(|plant| plants_vector.push(plant_names[&plant]));

	plants_vector
}
