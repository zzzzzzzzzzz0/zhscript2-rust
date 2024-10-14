use super::{*, equ_name_::equ_name__, super::as_mut_ref__};

pub const ARG_:&str = "参数";
pub const ARGS_:&str = "参数栈";
pub const ARGC_:&str = "参数数目";

pub struct Argi_ {
	super_:Item1_,
	i_:i32,
}
impl Argi_ {
	pub fn new(kws:&keyword_::List_, rems:Vec<String>, i_:i32) -> Self {
		Self {super_:Item1_::new(&kws, rems), i_}
	}
	pub fn with__(name:&str) -> Option<i32> {
		if name.starts_with(ARG_) {
			if let Ok(i) = name[ARG_.len()..].parse::<i32>() {
				return Some(i)
			}
		}
		None
	}

	pub fn hello__(i:i32, is_has:bool, equ_name:bool, name:&str, q:qv_::T_, ret:&mut result_::List_) {
		let q = as_ref__!(q);
		let a = &as_ref__!(q.args_);
		let i = t_::i2u__(i, a.len__());
		if i <= a.len__() {
			#[allow(clippy::never_loop)]
			loop {
				if is_has {
					ret.add__("1");
					break;
				}
				let i = i - 1;
				if equ_name {
					if let Some(s) = a.s3__(i) {
						if equ_name__(&s, name, ret) {
							break;
						}
						ret.add__(s);
						break;
					}
				}
				a.ret__(i, ret);
				break;
			}
		}
	}
}
impl code_::Item_ for Argi_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.super_.kw__()}
	fn s__(&self, ret:&mut String, w:&World_) {
		self.super_.s__(self.name__(w), ret, w)
	}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		self.super_.hello__(|q2| {
			Self::hello__(self.i_, false, false, "", q2.unwrap(), &mut as_mut_ref__!(env.ret));
			ok__()
		}, self, env)
	}
}
impl Name_ for Argi_ {
	fn name__(&self, _:&World_) -> String {format!("{}{}", ARG_, self.i_)}
}

pub struct Arg0_ {
	super_:Item1_,
	zero_:String,
}
impl Arg0_ {
	pub fn new(kws:&keyword_::List_, rems:Vec<String>, zero:&str) -> Self {
		Self {super_:Item1_::new(&kws, rems), zero_:zero.to_string()}
	}
	pub fn hello__(zero:&str, is_has:bool, q:qv_::T_, ret:&mut result_::List_) -> Result2_ {
		if is_has {
			ret.add__("1")
		} else {
			match zero {
				"0" | "00" => {
					let q = as_ref__!(q);
					if !q.arg0_.is_empty() {
						ret.add__(&q.arg0_);
					} else if zero == "0" || q.src_is_file_ {
						ret.add__(&q.src_);
					}
				}
				"000" => {
					let mut b = true;
					for i in &as_ref__!(q).name_ {
						if b {
							b = false;
						} else {
							ret.add__('+');
						}
						ret.add__(i);
					}
				}
				_ => {}
			}
		}
		ok__()
	}
}
impl code_::Item_ for Arg0_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.super_.kw__()}
	fn s__(&self, ret:&mut String, w:&World_) {self.super_.s__(self.name__(w), ret, w)}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		self.super_.hello__(|q2| {
			Self::hello__(&self.zero_, false, q2.unwrap(), &mut as_mut_ref__!(env.ret))
		}, self, env)
	}
}
impl Name_ for Arg0_ {
	fn name__(&self, _:&World_) -> String {[ARG_, &self.zero_].concat()}
}
