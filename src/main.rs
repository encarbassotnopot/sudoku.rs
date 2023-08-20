#![allow(unused, dead_code)]
mod sudoku;
use std::{fs, str::FromStr};

use sudoku::board::*;


fn main() {
	let sdk =
		Board::from_str(&fs::read_to_string("sudoku.txt").unwrap().replace("\n", "")).unwrap();
	let a = sdk.get_row(0).unwrap();
	let b = sdk.get_col(2).unwrap();
	let c = sdk.get_3x3(2, 1).unwrap().get_candidates();
	let d = a.get_candidates();
	dbg!(c);
	println!("{sdk:}");
}
