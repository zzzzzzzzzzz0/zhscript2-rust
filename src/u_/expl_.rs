#[derive(Debug, Clone)]
pub enum Item_ {
	N(f64),
	C(char),
}

#[derive(Default, Clone)]
pub struct List_ {
	pub a_:Vec<Item_>,
}

type Result_<'a> = Result<(f64, char), &'a str>;

impl List_ {
	fn item__(&self, idx:usize) -> Result_ {
		if idx < self.a_.len() {
			let i = &self.a_[idx];
			if let Item_::N(n) = i {
				return Ok((*n, 'n'));
			}
			if let Item_::C(c) = i {
				return Ok((0.0, *c));
			}
		}
		Err("缺")
	}
	pub fn z2__(&self, idx:&mut usize) -> Result_ {
		let mut ret = self.z3__(idx)?;
		while *idx < self.a_.len() {
			let (_, c) = self.item__(*idx)?;
			match c {
				'+' | '-' => {
					*idx += 1;
					let ret2 = self.z3__(idx)?;
					if c == '+' {
						ret.0 += ret2.0
					} else {
						ret.0 -= ret2.0
					}
				}
				_ => break
			}
		}
		Ok(ret)
	}
	fn z3__(&self, idx:&mut usize) -> Result_ {
		let mut ret = self.z4__(idx)?;
		while *idx < self.a_.len() {
			let (_, c) = self.item__(*idx)?;
			match c {
				'*' | '/' | '%' => {
					*idx += 1;
					let ret2 = self.z4__(idx)?;
					if ret2.0 == 0.0 && c != '*' {
						return Err("除零")
					}
					match c {
						'*' => ret.0 *= ret2.0,
						'/' => ret.0 /= ret2.0,
						_ => ret.0 = (ret.0 as i64 % ret2.0 as i64) as f64,
					}
				}
				_ => break
			}
		}
		Ok(ret)
	}
	fn z4__(&self, idx:&mut usize) -> Result_ {
		let mut ret = self.z5__(idx)?;
		while *idx < self.a_.len() {
			let (_, c) = self.item__(*idx)?;
			match c {
				'^' => {
					*idx += 1;
					let ret2 = self.z4__(idx)?;
					ret.0 = ret.0.powf(ret2.0);
				}
				_ => break
			}
		}
		Ok(ret)
	}
	fn z5__(&self, idx:&mut usize) -> Result_ {
		let i = self.item__(*idx)?;
		let c = i.1;
		match c {
			'n' => {
				*idx += 1;
				Ok(i)
			}
			'(' => {
				*idx += 1;
				let ret = self.z2__(idx)?;
				let (_, c2) = self.item__(*idx)?;
				if c2 == ')' {
					*idx += 1;
					return Ok(ret)
				}
				Err("应遇 )")
			}
			'-' | '+' => {
				*idx += 1;
				let ret = self.z3__(idx)?;
				if c == '-' {
					return Ok((-ret.0, 'n'))
				}
				Ok(ret)
			}
			_ => Err("错遇 ")
		}
	}
}
