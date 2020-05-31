use super::{*, super::{as_mut_ref__}};

const ARG_:&str = "参数";

pub struct Arg_ {
	super_:Item1_,
}
impl Arg_ {
	pub fn new(kws:&keyword_::List_, rems:Vec<String>) -> Self {
		Self {super_:Item1_::new(&kws, rems)}
	}
	pub fn with__(name:&str) -> bool {name == ARG_}

	pub fn hello__(is_has:bool, q:qv_::T_, ret:&mut result_::List_) -> Result2_ {
		if is_has {
			ret.add__("1")
		} else {
			ret.add__(as_mut_ref__!(q).args_1__())
		}
		ok__()
	}
}
impl code_::Item_ for Arg_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.super_.kw__()}
	fn s__(&self, ret:&mut String, w:&World_) {self.super_.s__(ARG_, ret, w)}
	fn hello__(&self, _gd:code_::Opt_, q:qv_::T_, w:world_::T_, _wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		match self.super_.super_.qv4rem__(&self.super_.rems_, |_| {false}, q, w) {
			Ok(q2) =>
				Self::hello__(false, q2.unwrap(), ret),
			Err(e) =>
				e,
		}
	}
}

pub struct Argi_ {
	super_:Item1_,
	i_:usize,
}
impl Argi_ {
	pub fn new(kws:&keyword_::List_, rems:Vec<String>, i_:usize) -> Self {
		Self {super_:Item1_::new(&kws, rems), i_}
	}
	pub fn with__(name:&str) -> Option<usize> {
		if name.starts_with(ARG_) {
			if let Ok(i) = name[ARG_.len()..].parse::<usize>() {
				return Some(i)
			}
		}
		None
	}

	pub fn hello__(i:usize, is_has:bool, q:qv_::T_, ret:&mut result_::List_) -> Result2_ {
		let a = &as_ref__!(q).args_;
		if i <= a.len__() {
			if is_has {
				ret.add__("1")
			} else {
				a.ret__(i - 1, ret);
			}
		}
		ok__()
	}
}
impl code_::Item_ for Argi_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.super_.kw__()}
	fn s__(&self, ret:&mut String, w:&World_) {
		self.super_.s__(&format!("{}{}", ARG_, self.i_), ret, w)
	}
	fn hello__(&self, _gd:code_::Opt_, q:qv_::T_, w:world_::T_, _wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		match self.super_.super_.qv4rem__(&self.super_.rems_, |_| {false}, q, w) {
			Ok(q2) =>
				Self::hello__(self.i_, false, q2.unwrap(), ret),
			Err(e) =>
				e,
		}
	}
}

pub struct Arg0_ {
	super_:Item1_,
}
impl Arg0_ {
	pub fn new(kws:&keyword_::List_, rems:Vec<String>) -> Self {
		Self {super_:Item1_::new(&kws, rems)}
	}
	pub fn hello__(is_has:bool, q:qv_::T_, ret:&mut result_::List_) -> Result2_ {
		if is_has {
			ret.add__("1")
		} else {
			ret.add__(&as_ref__!(q).src_)
		}
		ok__()
	}
}
impl code_::Item_ for Arg0_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.super_.kw__()}
	fn s__(&self, ret:&mut String, w:&World_) {self.super_.s__(&[ARG_, "0"].concat(), ret, w)}
	fn hello__(&self, _gd:code_::Opt_, q:qv_::T_, w:world_::T_, _wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		match self.super_.super_.qv4rem__(&self.super_.rems_, |_| {false}, q, w) {
			Ok(q2) =>
				Self::hello__(false, q2.unwrap(), ret),
			Err(e) =>
				e,
		}
	}
}
