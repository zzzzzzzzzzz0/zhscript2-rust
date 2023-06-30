#[derive(Default)]
pub struct IsText_ {
	text_:i32,
	text2_:i32,
	pub yuanyang_:i32,
	code_:i32,
	undef_:i32,
	rem_:i32,
	rem2_:i32,
	var_:i32,

	pub data_:bool,
}

impl IsText_ {
	pub fn sp__(c:u8) -> bool {
		c <= b' '
	}
	
	pub fn     text__(&self) -> bool {0 < self.text_}
	pub fn yuanyang__(&self) -> bool {0 < self.yuanyang_}
	pub fn     code__(&self) -> bool {0 < self.code_}
	pub fn     data__(&self) -> bool {    self.data_}
	pub fn    text2__(&self) -> bool {0 < self.text2_}
	pub fn      rem__(&self) -> bool {0 < self.rem_}
	pub fn     rem2__(&self) -> bool {0 < self.rem2_}
	pub fn      var__(&self) -> bool {0 < self.var_}
	pub fn    undef__(&self) -> bool {
		0 < self.undef_
	}
	
	pub fn add__(&self, c:u8) -> bool {
		if self.rem_ > 0 {
			return false
		}
		if self.yuanyang_ > 0 {
			return true
		}
		if self.text_ > 0 || self.var_ > 0 {
			return c >= b' '
		}
		if self.text2_ > 0 {
			return true
		}
		c > b' '
	}
	
	pub fn add2__(&mut self, c:char, c2:char) -> i32 {self.add4__(c, c2, 2)}
	pub fn add3__(&mut self, c:char, c2:char) -> i32 {self.add4__(c, c2, 1)}
	fn add4__(&mut self, c:char, c2:char, i2:i32) -> i32 {
		let i = match c {
			'r' => &mut self.rem_,
			't' => &mut self.text_,
			'y' => &mut self.yuanyang_,
			'c' => &mut self.code_,
			'e' => &mut self.text2_,
			_ => panic!("{}{}", c, c2),
		};
		let b = if c2 == '+' {
			*i += 1;
			*i > 1
		} else {
			*i -= 1;
			*i > 0
		};
		if self.code__() {
			match c {
				'c' | 'r' => {}
				_ => return 2
			}
		}
		if self.text2__() {
			match c {
				'e' | 'r' => {}
				_ => return 2
			}
		}
		if b {i2} else {1}
	}
	
	pub fn var2__(&mut self) {self.var_ += 1}
	pub fn var3__(&mut self) {self.var_ -= 1}
	
	pub fn rem22__(&mut self) {self.rem2_ += 1}
	pub fn rem23__(&mut self) {self.rem2_ -= 1}
	
	pub fn as2__(&self) -> bool {
		self.yuanyang__() || self.code__()
	}
	
	pub fn need_clear__(&mut self) -> bool {
		let text = if self.text__() || self.text2__() || self.as2__() || self.var__() || self.rem2__() {-1} else {1};
		match self.undef_ {
			0 => {
				self.undef_ = text;
				false
			}
			_ => self.undef_ != text,
		}
	}
	pub fn clear__(&mut self) {
		self.undef_ = 0;
	}
}

