#![allow(unused, dead_code)]
mod sudoku;
use std::{fs, str::FromStr};

use sudoku::board::*;
use sudoku::tile::*;

use crate::sudoku::solver::solve;


fn main() {
	let mut sdk =
		Board::from_str(&fs::read_to_string("sudoku.txt").unwrap().replace("\n", "")).unwrap();
		println!("{sdk}");
	println!("{sdk}");
	solve(&mut sdk);
	println!("{sdk}");
}
