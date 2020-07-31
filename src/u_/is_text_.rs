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
	
	pub fn     text__(&self) -> bool {self.text_ > 0}
	pub fn yuanyang__(&self) -> bool {self.yuanyang_ > 0}
	pub fn     code__(&self) -> bool {self.code_ > 0}
	pub fn     data__(&self) -> bool {self.data_}
	pub fn    text2__(&self) -> bool {self.text2_ > 0}
	pub fn    undef__(&self) -> bool {self.undef_ > 0}
	pub fn      rem__(&self) -> bool {self.rem_ > 0}
	pub fn     rem2__(&self) -> bool {self.rem2_ > 0}
	pub fn      var__(&self) -> bool {self.var_ > 0}
	
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
	
	pub fn add2__(&mut self, s:&str) -> i32 {self.add4__(s, 2)}
	pub fn add3__(&mut self, s:&str) -> i32 {self.add4__(s, 1)}
	fn add4__(&mut self, s:&str, i2:i32) -> i32 {
		let c = s.as_bytes()[0];
		let i = match c {
			b'r' => &mut self.rem_,
			b't' => &mut self.text_,
			b'y' => &mut self.yuanyang_,
			b'c' => &mut self.code_,
			b'e' => &mut self.text2_,
			_ => panic!("{}", s),
		};
		let b = if s.ends_with('+') {
			*i += 1;
			*i > 1
		} else {
			*i -= 1;
			*i > 0
		};
		if self.code__() {
			match c {
				b'c' | b'r' => {}
				_ => return 2
			}
		}
		if self.text2__() {
			match c {
				b'e' | b'r' => {}
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
		let text = if self.text__() || self.as2__() || self.var__() || self.rem2__() {-1} else {1};
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

