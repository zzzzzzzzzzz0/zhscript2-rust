use super::u_::*;

pub struct Item_ {
	kw_:keyword_::RI_,
	a_:code_::OL_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {kw_:kws.continue2_.clone(), a_:None}
	}
}

impl jump_::Item_ for Item_ {
	fn i__(&self) -> i32 {jump_::CONTINUE2_}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {&self.kw_}
	fn add__(&mut self, a:code_::List_) -> Result2_ {
		self.a_ = Some(a);
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}
	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		jump_::Item_::hello__(self, gd, q, w, ret)
	}
}