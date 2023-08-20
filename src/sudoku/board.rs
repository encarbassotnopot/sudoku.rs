use std::{fmt, str};
use super::errors::*;
use super::collection::*;


#[derive(Clone, Debug)]
pub struct Board {
	board: Vec<u8>,
}

impl Board {
	pub fn get_board(&self) -> &Vec<u8> {
		&self.board
	}

	pub fn get_row(&self, index: usize) -> Result<SudokuCollection, BoardOutOfBounds> {
		match index < 9 {
			true => Ok(SudokuCollection::from(self.get_board().iter().skip(index * 9).take(9).collect::<Vec<&u8>>())),
			false => Err(BoardOutOfBounds),
		}
	}
	
	fn get_row_private(&self, index: usize) -> Result<Vec<&u8>, BoardOutOfBounds>{
		match index < 9 {
			true => Ok(self.get_board().iter().skip(index * 9).take(9).collect::<Vec<&u8>>()),
			false => Err(BoardOutOfBounds),
		}
	}

	pub fn get_col(&self, index: usize) -> Result<SudokuCollection, BoardOutOfBounds> {
		match index < 9 {
			true => Ok(SudokuCollection::from(self.get_board().iter().skip(index).step_by(9).collect::<Vec<&u8>>())),
			false => Err(BoardOutOfBounds),
		}
	}

	pub fn get_3x3(&self, x: usize, y: usize) -> Result<SudokuCollection, BoardOutOfBounds> {
		match x < 3 && y < 3 {
			true => {
				let mut col: Vec<&u8> = Vec::new();
				for i in (0..3) {
					for j in (0..3) {
						col.push(self.get_row_private(y * 3 + i)?.get(x * 3 + j).unwrap())
					}
				}
				let col: SudokuCollection = col.into();
				Ok(col)
			}
			false => Err(BoardOutOfBounds),
		}
	}
}

impl str::FromStr for Board {
	type Err = InvalidSudokuString;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.len() != 81 {
			return Err(InvalidSudokuString);
		}

		let sdk = s
			.chars()
			.map(|num| num.to_digit(10).unwrap() as u8)
			.collect::<Vec<u8>>();

		Ok(Self { board: sdk })
	}
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		const TOP: &str = "┏━━━┯━━━┯━━━┳━━━┯━━━┯━━━┳━━━┯━━━┯━━━┓\n";
		const MID: &str = "┣━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━┫\n";
		const SEP: &str = "┠───┼───┼───╂───┼───┼───╂───┼───┼───┨\n";
		const BOT: &str = "┗━━━┷━━━┷━━━┻━━━┷━━━┷━━━┻━━━┷━━━┷━━━┛";

		let mut display = String::new();
		String::reserve_exact(&mut display, 939);

		display += TOP;
		self.board.chunks(9).enumerate().for_each(|(idx, row)| {
			const THICK: &str = "┃";
			const THIN: &str = "│";

			let mut line = THICK.to_string();
			row.iter().enumerate().for_each(|(r_idx, el)| {
				line += " ";
				if el.clone() != 0 {
					line += &el.to_string();
				} else {
					line += " ";
				}
				line += " ";

				if (r_idx + 1) % 3 == 0 {
					line += THICK;
				} else {
					line += THIN;
				}

				if r_idx == 8 {
					line += "\n";
				}
			});

			display += &line;

			if idx == 2 || idx == 5 {
				display += MID;
			} else if idx != 8 {
				display += SEP;
			}
		});
		display += BOT;

		write!(f, "{}", display)
	}
}
