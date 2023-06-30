use super::u_::*;

pub struct Item_ {
	super_:code_::Item1_,
	s_:String,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_, s_:String) -> Self {
		Self {super_:code_::Item1_::new(&kws.undef_), s_}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn s__(&self, ret:&mut String, _:&World_) {
		ret.push_str(&self.s_);
	}
	fn s2__(&self) -> &str {&self.s_}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		as_mut_ref__!(env.ret).add__(&self.s_);
		ok__()
	}
}
