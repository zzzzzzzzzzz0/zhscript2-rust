use super::u_::*;

pub struct Item_ {
	kw_:keyword_::RI_,
	a_:code_::OL_,
	i_:i32,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_, i_:i32) -> Self {
		Self {kw_:match i_ {
			jump_::BREAK_  => kws.break_.clone(),
			jump_::BREAK2_ => kws.break2_.clone(),
			_ => panic!()
		}, a_:None, i_}
	}
}

impl jump_::Item_ for Item_ {
	fn i__(&self) -> i32 {self.i_}
	fn b__(&self) -> bool {false}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {&self.kw_}
	fn add__(&mut self, a:code_::List_) -> Result2_ {
		self.a_ = Some(a);
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		jump_::Item_::hello__(self, env)
	}
}