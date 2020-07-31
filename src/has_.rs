use super::{u_::*, *};

pub struct Item_ {
	super_:var_::Item_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {
			super_:var_::Item_::new2(
				var_::Item1_::new2(
					code_::Item1_::new(&kws.has_),
					vec![]
				)
			)
		}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.super_.super_.kw__()}
	fn kw2__(&self) -> keyword_::ORI_ {self.super_.super_.super_.kw2__()}
	fn add__(&mut self, a:code_::List_) -> Result2_ {self.super_.add__(a)}
	fn a__(&self) -> code_::ORL_ {self.super_.a__()}

	fn hello__(&self, env:&code_::Env_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		let mut ret2 = result_::List_::new();
		t_::o__(&self.a__()).hello__(env, wm, &mut ret2)?;
		self.super_.super_.get__(&ret2, true, env, ret)
	}
}