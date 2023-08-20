use super::{board::*, collection::SudokuCollection};

pub fn solve(board: &mut Board) {

	while (!board.is_solved()) {

		let mut rows = Vec::new();
		let mut cols = Vec::new();
		let mut squares = Vec::new();

		for x in (0..9) {
			rows.push(board.get_row(x).unwrap().get_candidates())
		}

		for y in (0..9) {
			cols.push(board.get_col(y).unwrap().get_candidates())
		}

		for y in (0..3) {
			for x in (0..3) {
				squares.push(board.get_3x3(x, y).unwrap().get_candidates())
			}
		}

		for x in (0..9) {
			for y in (0..9) {
				let tile = board.get_tile_mut(x, y).unwrap();

				if (tile.get_number().is_none()) {					
					let mut candidates = get_candidates(&rows[y], &cols[x], &squares[(y/3)*3 + x/3]);
					tile.set_candidates(&candidates);
				}
			}
		}
	}
}

fn get_candidates(a: &Vec<u8>, b: &Vec<u8>, c: &Vec<u8>) -> Vec<u8> {
	let mut candidates = Vec::new();

	for i in (1..10) {
		if a.contains(&i) && b.contains(&i) && c.contains(&i) {
			candidates.push(i)
		}
	}

	candidates
}