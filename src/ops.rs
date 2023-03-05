use core::ops::{Add, AddAssign};

use crate::BigInt;

impl<'a> Add<u32> for &'a BigInt {
	type Output = BigInt;

	fn add(self, other: u32) -> Self::Output {
		let mut ret: BigInt = self.clone();
		ret += other;
		return ret;
	}
} 
impl<'a> Add<&'a BigInt> for &'a BigInt {
	type Output = BigInt;

	fn add(self, other: &'a BigInt) -> Self::Output {
		let mut ret = self.clone();
		ret += other;
		return ret;
	}
}

impl AddAssign<u32> for BigInt {
	fn add_assign(&mut self, other: u32) {
		*self += &BigInt::new(other);
	}
}

impl<'a> AddAssign<&'a BigInt> for BigInt {
	fn add_assign(&mut self, other: &'a BigInt) {
		self.val += other.val;
	}
}

pub fn add_assign_byte(a: &mut [u32], b: u32) -> bool {
	let (v, c) = a[0].overflowing_add(b);
	a[0] = v;

	if a.len() > 1 {
		return add_assign_byte(&mut a[1..], c as u32);
	} else {
		return c;
	}
}