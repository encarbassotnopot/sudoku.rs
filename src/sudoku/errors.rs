use std::fmt;

#[derive(Clone, Debug)]
pub struct InvalidSudokuString;
impl fmt::Display for InvalidSudokuString {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"The given string can't be translated into a valid board."
		)
	}
}

#[derive(Clone, Debug)]
pub struct BoardOutOfBounds;
impl fmt::Display for BoardOutOfBounds {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"The given string can't be translated into a valid board."
		)
	}
}
