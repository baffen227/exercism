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
		let mut result: Vec<u32> = Vec::new();
		let mut heap2 = self.heap.clone();
		match self.heap.len() {
			0 => result,
			1 => {
				result.push(heap2.peek().copied().unwrap());
				result
			}
			2 => {
				result.push(heap2.pop().unwrap());
				result.push(heap2.pop().unwrap());
				result
			}
			_ => {
				result.push(heap2.pop().unwrap());
				result.push(heap2.pop().unwrap());
				result.push(heap2.pop().unwrap());
				result
			}
		}
	}
}
