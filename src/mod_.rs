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
	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		name_::Item1_::hello__(self, gd, q, w, wm, ret)
	}
}

impl name_::Item1_ for Item_ {
	fn codes2__(&mut self, a:code_::OL_) {self.a_ = a}
	fn codes__(&self) -> &code_::OL_ {&self.a_}
	fn can__(&self, name:&String, q:qv_::T_, w:world_::T_) -> bool {
		if as_ref__!(w).mods_.iter().any(|q| as_ref__!(q).name_.contains(name)) {
			false
		} else {
			as_mut_ref__!(w).mods_.push(q);
			true
		}
	}
}
