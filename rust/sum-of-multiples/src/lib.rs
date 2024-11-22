use std::collections::BTreeSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
	let mut results: BTreeSet<u32> = BTreeSet::new();
	factors
		.iter()
		.filter(|i| **i != 0)
		.for_each(|factor| {
			let multiples: BTreeSet<u32> = (*factor..limit)
				.step_by(*factor as usize)
				.collect();
			results = results
				.union(&multiples)
				.cloned()
				.collect();
		});
	results.iter().sum::<u32>()
}
