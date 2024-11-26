use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct HighScores {
	vector: Vec<u32>,
	heap: BinaryHeap<u32>,
}

impl HighScores {
	pub fn new(scores: &[u32]) -> Self {
		HighScores {
			vector: scores.to_vec(),
			heap: BinaryHeap::from(scores.to_vec()),
		}
	}

	pub fn scores(&self) -> &[u32] {
		&self.vector
	}

	pub fn latest(&self) -> Option<u32> {
		self.vector.last().copied()
	}

	pub fn personal_best(&self) -> Option<u32> {
		self.heap.peek().copied()
	}

	pub fn personal_top_three(&self) -> Vec<u32> {
		// TODO: use peek()
		let result = self.heap.clone().into_vec();
		match result.len() {
			0..=2 => result,
			_ => result[0..=2].to_vec(),
		}
	}
}
