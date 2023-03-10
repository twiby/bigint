#[cfg(test)]
mod test;

pub struct Digits {
	val: Vec<u8>
}

impl Digits {
	pub fn new(mut val: usize) -> Digits {
		let mut ret = Digits{val: Vec::new() };

		while val > 0 {
			ret.val.push( (val % 10).try_into().unwrap() );

			val -= *ret.val.iter().last().unwrap() as usize;
			val /= 10;
		}

		return ret;
	}

	pub fn add_n_at_k(&mut self, n: u8, k: usize) {
		if n == 0 {
			return;
		} else if k == self.val.len() {
			self.val.push(n);
		} else {
			let new_v = self.val[k] + n;
			self.val[k] = new_v % 10;
			self.add_n_at_k((new_v - self.val[k])/10, k+1);
		}
	}

	pub fn times_2(&mut self) {
		let digits = self.val.clone();
		for i in 0..digits.len() {
			self.add_n_at_k(digits[i], i);
		}
	}
}

impl From<&str> for Digits {
	fn from(other: &str) -> Digits {
		let mut ret = Digits::new(0);

		for c in other.chars().rev() {
			ret.val.push(c.to_digit(10).unwrap().try_into().unwrap());
		}

		ret
	}
}

impl From<&Digits> for String {
	fn from(other: &Digits) -> String {
		let mut ret = "".to_string();
		for c in other.val.iter().rev() {
			ret.push(char::from_digit((*c).into(), 10).unwrap());
		}
		return ret;
	}
}