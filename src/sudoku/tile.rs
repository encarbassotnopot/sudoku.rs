use std::fmt::Display;


#[derive(Debug, Clone)]
pub struct Tile {
	number: Option<u8>,
	candidates: Vec<u8>,
}

impl Tile {
	pub fn new(number: u8) -> Tile {
		Tile { number: {
			if number == 0 {
				None
			} else {
				Some(number)
			}
		}, candidates: Vec::new() }
	}

	pub fn get_number(&self) -> Option<u8> {
		self.number
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