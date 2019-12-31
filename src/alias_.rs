use super::u_::*;
use super::*;
/*use std::ops::Deref;
use std::ops::DerefMut;*/

pub struct Item_ {
	super_:set_::Item_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:set_::Item_::new2(&kws.alias_, &kws.equ_)}
	}
}

/*impl Deref for Item_ {
	type Target = dyn code_::Item_;
	fn deref(&self) -> &Self::Target {&self.super_}
}
impl DerefMut for Item_ {
	fn deref_mut(&mut self) -> &mut (dyn code_::Item_ + 'static) {&mut self.super_}
}*/

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn kw2__(&self) -> keyword_::ORI_ {self.super_.kw2__()}

	fn add__(&mut self, a:code_::List_) -> Result2_ {self.super_.add__(a)}
	fn add2__(&mut self, a:code_::List_) -> Result2_ {self.super_.add2__(a)}
	fn a__(&self) -> code_::ORL_ {self.super_.a__()}
	fn a2__(&self) -> code_::ORL_ {self.super_.a2__()}

	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		self.super_.hello2__(true, gd, q, w, ret)
	}
}
