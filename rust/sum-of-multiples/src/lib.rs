use std::collections::BTreeSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
	let results: BTreeSet<u32> = factors
		.iter()
		.filter(|i| **i != 0)
		.fold(BTreeSet::new(), |results, factor| {
			let multiples: BTreeSet<u32> = (*factor..limit)
				.step_by(*factor as usize)
				.collect();
			results
				.union(&multiples)
				.cloned()
				.collect()
		});
	results.iter().sum::<u32>()
}
