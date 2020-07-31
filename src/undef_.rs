use super::u_::*;

pub struct Item_ {
	super_:code_::Item1_,
	s_:String,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_, s:&str) -> Self {
		Self {super_:code_::Item1_::new(&kws.undef_), s_:s.to_string()}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn s__(&self, ret:&mut String, _w:&World_) {
		ret.push_str(&self.s_);
	}
	fn s2__(&self) -> &str {&self.s_}
	fn hello__(&self, _env:&code_::Env_, _wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		ret.add__(&self.s_);
		ok__()
	}
}
