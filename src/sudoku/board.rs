use super::collection::*;
use super::errors::*;
use super::tile::*;
use std::{fmt, str};

#[derive(Clone, Debug)]
pub struct Board {
	board: Vec<Tile>,
}

impl Board {
	pub fn new() -> Board {
		Board { board: Vec::new() }
	}
	pub fn get_board(&self) -> &Vec<Tile> {
		&self.board
	}

	pub fn get_row(&self, index: usize) -> Result<SudokuCollection, BoardOutOfBounds> {
		match index < 9 {
			true => Ok(SudokuCollection::from(
				self.board
					.iter()
					.skip(index * 9)
					.take(9)
					.collect::<Vec<&Tile>>(),
			)),
			false => Err(BoardOutOfBounds),
		}
	}

	fn get_row_private(&self, index: usize) -> Result<Vec<&Tile>, BoardOutOfBounds> {
		match index < 9 {
			true => Ok(self
				.board
				.iter()
				.skip(index * 9)
				.take(9)
				.collect::<Vec<&Tile>>()),
			false => Err(BoardOutOfBounds),
		}
	}

	pub fn get_col(&self, index: usize) -> Result<SudokuCollection, BoardOutOfBounds> {
		match index < 9 {
			true => Ok(SudokuCollection::from(
				self.board
					.iter()
					.skip(index)
					.step_by(9)
					.collect::<Vec<&Tile>>(),
			)),
			false => Err(BoardOutOfBounds),
		}
	}

	pub fn get_3x3(&self, x: usize, y: usize) -> Result<SudokuCollection, BoardOutOfBounds> {
		match x < 3 && y < 3 {
			true => {
				let mut col: Vec<&Tile> = Vec::new();
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

	pub fn get_tile(&self, x: usize, y: usize) -> Result<&Tile, BoardOutOfBounds> {
		match x < 9 && y < 9 {
			true => Ok(&self.board[y + x * 9]),
			false => Err(BoardOutOfBounds),
		}
	}

	pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Result<&mut Tile, BoardOutOfBounds> {
		match x < 9 && y < 9 {
			true => Ok(self.board.get_mut(x + y * 9).unwrap()),
			false => Err(BoardOutOfBounds),
		}
	}

	pub fn is_solved(&self) -> bool {
		self.board
			.iter()
			.fold(true, |solved, tile| solved && tile.get_number().is_some())
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
			.map(|num| Tile::new(num.to_digit(10).unwrap() as u8))
			.collect::<Vec<Tile>>();

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
				line += &el.to_string();
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
