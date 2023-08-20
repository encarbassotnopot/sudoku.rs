use core::num;
use std::{slice::SliceIndex, ops::Range};

use super::tile::Tile;

#[derive(Clone, Debug)]
pub struct SudokuCollection<'a> {
	collection: Vec<&'a Tile>,
}

impl<'a> From<Vec<&'a Tile>> for SudokuCollection<'a> {
    fn from(value: Vec<&'a Tile>) -> Self {
        Self {
			collection: value
		}
    }
}

impl SudokuCollection<'_> {
	pub fn get_candidates(&self) -> Vec<u8> {
		let numbers: Vec<u8> = self.collection.iter().filter_map(|t| *t.get_number()).collect();
		let mut candidates = Vec::new();

		for i in (1..10) {
			if !numbers.contains(&i) {
				candidates.push(i)
			}
		}

		candidates
	}
	
	pub fn get(&self, index: usize) -> &Tile {
		self.collection[index]
	}

	pub fn contents(&self) -> &Vec<&Tile> {
		&self.collection
	}
}
