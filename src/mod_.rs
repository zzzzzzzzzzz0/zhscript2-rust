use super::{u_::*, name_};

pub struct Item_ {
	super_:code_::Item1_,
	a_:code_::OL_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:code_::Item1_::new(&kws.mod_), a_:None}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn add__(&mut self, a:code_::List_) -> Result2_ {name_::Item1_::add__(self, a)}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		name_::Item1_::hello__(self, env)
	}
}

impl name_::Item1_ for Item_ {
	fn codes2__(&mut self, a:code_::OL_) {self.a_ = a}
	fn codes__(&self) -> &code_::OL_ {&self.a_}
	fn can__(&self, name:&String, argv:&Vec<String>, q:qv_::T_, w:world_::T_) -> bool {
		let mut is_data = false;
		for i in argv.iter().skip(2) {
			match i.as_str() {
				"数据" => is_data = true,
				_ => return false
			}
		}
		if as_ref__!(w).mods_.iter().any(|q| as_ref__!(q).name_.contains(name)) {
			return false
		}
		if as_ref__!(w).datas_.iter().any(|q| as_ref__!(q).name_.contains(name)) {
			return false
		}
		if is_data {
			as_mut_ref__!(w).datas_.push(q);
		} else {
			as_mut_ref__!(w).mods_.push(q);
		}
		true
	}
}
