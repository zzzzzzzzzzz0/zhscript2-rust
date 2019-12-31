use super::u_::*;
use super::*;

pub struct Item_ {
	super_:set_::Item_,
	sp_:usize,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:set_::Item_::new2(&kws.def_, &kws.equ_), sp_:code_::Item1_::split2_0__()}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn kw2__(&self) -> keyword_::ORI_ {self.super_.kw2__()}

	fn add__(&mut self, a:code_::List_) -> Result2_ {
		self.super_.super_.chk_empty__(&a, "名缺")?;
		code_::Item1_::split2_1__(&a, &mut self.sp_);
		self.super_.add__(a)
	}
	fn add2__(&mut self, a:code_::List_) -> Result2_ {self.super_.add2__(a)}
	fn a__(&self) -> code_::ORL_ {self.super_.a__()}
	fn a2__(&self) -> code_::ORL_ {self.super_.a2__()}

	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:&mut World_, _ret:&mut result_::List_) -> Result2_ {
		let mut name = String::new();
		let mut argc = core::usize::MAX;
		let mut ret2 = result_::List_::new();
		code_::Item1_::split2_2__(self.a__(), self.sp_, &mut name, |rem| {
			if rem == "无参" {
				argc = 0;
				return true
			}
			false
		}, gd, q.clone(), w, &mut ret2)?;
		//w.dbg_.arg__(&ret2);
		let mut ret3 = result_::List_::new();
		t_::o__(&self.a2__()).hello__(gd, q.clone(), w, &mut ret3)?;
		if name.is_empty() {return self.super_.super_.err__("名空")}
		if !ret2.is_empty() {
			argc = ret2.dunhao_ + 1
		}
		qv_::def__(&name, &ret3.s__(), argc, Some(&ret2), q, w)
	}
}
