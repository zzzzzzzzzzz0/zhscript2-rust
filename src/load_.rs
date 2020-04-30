use super::{u_::*, eval_};

pub struct Item_ {
	super_:code_::Item1_,
	a_:code_::OL_,
	sp_:usize,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:code_::Item1_::new(&kws.eval_), a_:None, sp_:code_::Item1_::split2_0__()}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn add__(&mut self, a:code_::List_) -> Result2_ {eval_::Item1_::add__(self, a)}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}
	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		eval_::Item1_::hello__(self, gd, q, w, wm, ret)
	}
}

impl eval_::Item1_ for Item_ {
	fn codes2__(&mut self, a:code_::OL_) {self.a_ = a}
	fn codes__(&self) -> &code_::OL_ {&self.a_}
	fn sp2__(&mut self, i:usize) {self.sp_ = i}
	fn sp__(&self) -> usize {self.sp_}
	fn src__(&self, s:String, src2:&mut String, q:&mut Qv_, w:&World_, dbg:&mut Dbg_) -> Result2_ {
		super::u_::eval_::src__(&s, src2, q, w, dbg)
	}
}
