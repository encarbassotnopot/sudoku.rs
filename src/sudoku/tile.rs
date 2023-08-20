use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Tile {
	number: Option<u8>,
	candidates: Vec<u8>,
}

impl Tile {
	pub fn new(number: u8) -> Tile {
		Tile {
			number: {
				if number == 0 {
					None
				} else {
					Some(number)
				}
			},
			candidates: {
				if number == 0 {
					vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
				} else {
					Vec::new()
				}
			},
		}
	}

	pub fn get_number(&self) -> &Option<u8> {
		&self.number
	}

	pub fn set_number(&mut self, number: u8) {
		self.number = Some(number);
	}

	pub fn get_candidates(&self) -> &Vec<u8> {
		&self.candidates
	}

	pub fn set_candidates(&mut self, candidates: &Vec<u8>) {
		self.candidates = candidates.to_vec();
		
		if self.candidates.len() == 1 {
			self.number = Some(self.candidates[0]);
		}
	}
}

impl Display for Tile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut out = String::new();
		if self.number.is_some() {
			out += &self.number.unwrap().to_string();
		} else {
			out += " ";
		}
		write!(f, "{}", out)
	}
}
