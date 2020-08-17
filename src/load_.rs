use super::{u_::*, eval_};

pub struct Item_ {
	super_:code_::Item1_,
	a_:code_::OL_,
	sp_:usize,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:code_::Item1_::new(&kws.load_), a_:None, sp_:code_::Item1_::split2_0__()}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn add__(&mut self, a:code_::List_) -> Result2_ {eval_::Item1_::add__(self, a)}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		eval_::Item1_::hello__(self, env)
	}
}

impl eval_::Item1_ for Item_ {
	fn codes2__(&mut self, a:code_::OL_) {self.a_ = a}
	fn codes__(&self) -> &code_::OL_ {&self.a_}
	fn sp2__(&mut self, i:usize) {self.sp_ = i}
	fn sp__(&self) -> usize {self.sp_}
	fn src__(&self, s:String, src2:&mut String, once:&mut bool, q:qv_::T_, w:world_::T_) -> Result2_ {
		super::u_::eval_::ok_src__(&s, q.clone(), w.clone());
		*once = as_ref__!(w).mods_.iter().any(|q2| as_ref__!(q2).src_ == as_ref__!(q).src_);
		if *once {
			return ok__()
		}
		super::u_::eval_::src__(src2, q, w)
	}
}
