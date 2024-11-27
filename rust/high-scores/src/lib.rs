use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct HighScores {
	all_scores: Vec<u32>,
}

impl HighScores {
	pub fn new(scores: &[u32]) -> Self {
		HighScores {
			all_scores: scores.to_vec(),
		}
	}

	pub fn scores(&self) -> &[u32] {
		&self.all_scores
	}

	pub fn latest(&self) -> Option<u32> {
		self.all_scores
			.last()
			.copied()
	}

	pub fn personal_best(&self) -> Option<u32> {
		let heap = BinaryHeap::from(self.all_scores.to_vec());
		heap.peek().copied()
	}

	pub fn personal_top_three(&self) -> Vec<u32> {
		let mut result: Vec<u32> = Vec::new();
		let mut heap = BinaryHeap::from(self.all_scores.to_vec());
		while !heap.is_empty() && result.len() < 3 {
			result.push(heap.pop().unwrap());
		}
		result
	}
}
