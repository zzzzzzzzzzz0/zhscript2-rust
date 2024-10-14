use super::{*, super::{as_ref__}};

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
			ret.add__(as_ref__!(as_ref__!(q).args_).len__())
		}
		ok__()
	}
}

impl code_::Item_ for Argc_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.super_.kw__()}
	fn s__(&self, ret:&mut String, w:&World_) {self.super_.s__(self.name__(w), ret, w)}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		self.super_.hello__(|q2| {
			Self::hello__(false, q2.unwrap(), &mut as_mut_ref__!(env.ret))
		}, self, env)
	}
}

impl Name_ for Argc_ {
	fn name__(&self, _:&World_) -> String {ARGC_.to_string()}
}
