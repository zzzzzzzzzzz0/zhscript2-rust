use super::{u_::*, eval_::*};

pub struct Item_ {
	super_:ItemA_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:ItemA_::new(&kws.load_)}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.super_.kw__()}
	fn add__(&mut self, a:code_::List_) -> Result2_ {Item1_::add__(self, a)}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.super_.a_)}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		Item1_::hello__(self, env)
	}
}

impl Item1_ for Item_ {
	fn super__(&self) -> &ItemA_ {&self.super_}
	fn super_mut__(&mut self) -> &mut ItemA_ {&mut self.super_}
	fn src__(&self, s:String, src2:&mut String, once:&mut bool, q:qv_::T_, w:world_::T_) -> Result2_ {
		super::u_::eval_::ok_src__(&s, q.clone(), w.clone());
		*once = as_ref__!(w).mods_.iter().any(|q2| as_ref__!(q2).src_ == as_ref__!(q).src_);
		if *once {
			return ok__()
		}
		super::u_::eval_::src__(src2, q, w)
	}
}
