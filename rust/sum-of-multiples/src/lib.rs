use std::collections::BTreeSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
	let mut results: BTreeSet<u32> = BTreeSet::new();
	for factor in factors {
		if *factor != 0 {
			let mut multiples = BTreeSet::new();
			let mut n = 1;
			let mut multiple = factor * n;
			while multiple < limit {
				multiples.insert(multiple);
				n += 1;
				multiple = factor * n;
			}
			results = results
				.union(&multiples)
				.cloned()
				.collect();
		}
	}
	results.iter().sum::<u32>()
}
