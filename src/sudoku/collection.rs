use core::num;
use std::{slice::SliceIndex, ops::Range};

#[derive(Clone, Debug)]
pub struct SudokuCollection<'a> {
	collection: Vec<&'a u8>,
}

impl<'a> From<Vec<&'a u8>> for SudokuCollection<'a> {
    fn from(value: Vec<&'a u8>) -> Self {
        Self {
			collection: value
		}
    }
}

impl SudokuCollection<'_> {
	pub fn get_candidates(&self) -> Vec<u8> {
		let mut collection = self.collection.clone();
		collection.sort_by(|a, b| b.cmp(a));

		let mut candidates: Vec<u8> = [1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec();
		
		collection.iter().take_while(|n| ***n != 0).for_each(|number| {
			candidates.remove((*number - 1).into());
		});
		
		candidates
	}
	
	pub fn get(&self, index: usize) -> &u8 {
		self.collection[index]
	}

	pub fn contents(&self) -> &Vec<&u8> {
		&self.collection
	}

}
