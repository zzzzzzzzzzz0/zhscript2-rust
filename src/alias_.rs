use super::{u_::*, *};

pub struct Item_ {
	super_:set_::Item_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:set_::Item_::new2(&kws.alias_, &kws.equ_)}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn kw2__(&self) -> keyword_::ORI_ {self.super_.kw2__()}

	fn add__(&mut self, a:code_::List_) -> Result2_ {self.super_.add__(a)}
	fn add2__(&mut self, a:code_::List_) -> Result2_ {self.super_.add2__(a)}
	fn a__(&self) -> code_::ORL_ {self.super_.a__()}
	fn a2__(&self) -> code_::ORL_ {self.super_.a2__()}

	fn hello__(&self, env:&code_::Env_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		self.super_.hello2__(true, env, wm, ret)
	}
}
