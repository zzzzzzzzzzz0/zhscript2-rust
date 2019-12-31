use super::*;

const ARGC_:&str = "参数数目";

pub struct Argc_ {
	super_:Item1_,
}

impl Argc_ {
	pub fn new(kws:&keyword_::List_, rems:Vec<String>) -> Self {
		Self {super_:Item1_::new(&kws, rems)}
	}
	pub fn with__(name:&str) -> bool {name == ARGC_}

	pub fn hello__(is_has:bool, q:qv_::T_, ret:&mut result_::List_) -> Result2_ {
		if is_has {
			ret.add__("1")
		} else {
			ret.add__(q.borrow().args2_.len())
		}
		ok__()
	}
}

impl code_::Item_ for Argc_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.super_.kw__()}
	fn s__(&self, ret:&mut String, w:&World_) {self.super_.s__(ARGC_, ret, w)}
	fn hello__(&self, _gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		match self.super_.super_.qv4rem__(&self.super_.rems_, |_| {false}, q, w) {
			Ok(q2) =>
				Self::hello__(false, q2.unwrap(), ret),
			Err(e) =>
				e,
		}
	}
}
