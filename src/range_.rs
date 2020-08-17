use super::{u_::*, for_};

pub struct Item_ {
	kw_:keyword_::RI_,
	pub codes_:code_::OL_,
	pub rems_:code_::List_,
	cnt_:for_::Cnt_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {kw_:kws.range_.clone(), codes_:None, rems_:code_::List_::new(), cnt_:for_::Cnt_::new()}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {&self.kw_}
	fn add__(&mut self, a:code_::List_) -> Result2_ {for_::Item1_::add__(self, a)}
	fn a__(&self) -> code_::ORL_ {for_::Item1_::a__(self)}
	fn a2__(&self) -> code_::ORL_ {for_::Item1_::a2__(self)}
	fn s__(&self, ret:&mut String, w:&World_) {for_::Item1_::s__(self, ret, w)}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		for_::Item1_::hello__(self, env)
	}
}

impl for_::Item1_ for Item_ {
	fn cnt2__(&mut self) ->  &mut for_::Cnt_ {&mut self.cnt_}
	fn codes2__(&mut self, a:code_::OL_) {self.codes_ = a}
	fn rems_push__(&mut self, i:code_::I_) {self.rems_.push(i)}
	fn cnt__(&self) ->  &for_::Cnt_ {&self.cnt_}
	fn rems__(&self) -> &code_::List_ {&self.rems_}
	fn codes__(&self) -> &code_::OL_ {&self.codes_}
	fn break__(&self) -> i32 {jump_::BREAK2_}
	fn continue__(&self) -> i32 {jump_::CONTINUE2_}
	fn loop__(&self) -> bool {false}
}
