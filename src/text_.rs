use super::u_::*;

pub struct Item_ {
	super_:code_::Item1_,
	s_:String,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_, s_:String) -> Self {
		let s1 = "续\n";
		let s2 = "\n续";
		let starts = s_.starts_with(s1);
		let ends = s_.ends_with(s2);
		let s_ = if starts || ends {
			s_[if starts {s1.len()} else {0}..s_.len() - if ends {s2.len()} else {0}].to_string()
		} else {s_};
		Self {super_:code_::Item1_::new2(&kws.begin_text_, &kws.end_text_), s_}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn s__(&self, ret:&mut String, _:&World_) {self.super_.s2__(&self.s_, ret)}
	fn s2__(&self) -> &str {&self.s_}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		as_mut_ref__!(env.ret).add__(&self.s_);
		ok__()
	}
}
