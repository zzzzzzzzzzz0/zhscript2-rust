use super::{*, super::{as_ref__}};

const ARGS_:&str = "参数栈";

pub struct Args_ {
	super_:Item1_,
	begin_:i32,
	end_:i32,
}

impl Args_ {
	pub fn new(kws:&keyword_::List_, rems:Vec<String>, begin_:i32, end_:i32) -> Self {
		Self {super_:Item1_::new(&kws, rems), begin_, end_}
	}
	pub fn with__(name:&str, rems:&mut Vec<String>) -> Option<(i32, i32)> {
		let mut begin = 1;
		let mut end = core::i32::MAX;
		let mut begin2 = core::usize::MAX;
		let mut end2 = core::usize::MAX;
		if name == ARGS_ {
			for (idx, i) in rems.iter().enumerate() {
				if let Some(i) = t_::s2n__(&i) {
					if begin2 == core::usize::MAX {
						begin2 = idx;
						begin = i;
					} else if end2 == core::usize::MAX {
						end2 = idx;
						end = i;
					} else {
						break
					}
				}
			}
			if end2 != core::usize::MAX {
				rems.remove(end2);
			}
			if begin2 != core::usize::MAX {
				rems.remove(begin2);
			}
			Some((begin, end))
		} else {
			None
		}
	}
	pub fn with2__(name:&str, rems:&Vec<String>) -> bool {
		if name == ARGS_ {
			let mut i2 = 0;
			for i in rems.iter() {
				if t_::s2n__::<i32>(&i).is_some() {
					i2 += 1;
					if i2 > 2 {return false}
				} else {
					return false
				}
			}
			true
		} else {false}
	}

	pub fn hello__(is_has:bool, q:qv_::T_, begin:i32, end:i32, ret:&mut result_::List_) -> Result2_ {
		let ls = &as_ref__!(q).args_;
		let a = &ls.a_;
		if !a.is_empty() {
			let len = ls.len__() + 2;
			let begin2 = t_::i2u__(begin, len);
			let end2 = t_::i2u__(end, len);
			if end2 >= begin2 {
				if is_has {
					ret.add__("1")
				} else {
					let mut i2 = 1;
					let mut len2 = 0;
					for i in a {
						if as_ref__!(i).dunhao__() {
							i2 += 1;
							if i2 == begin2 {
								continue
							}
						}
						if i2 < begin2 {
							continue
						}
						if i2 > end2 {
							break
						}
						ret.add4__(i.clone());
						len2 += 1;
					}
					if len2 == 0 {
						ret.pop_end__();
					}
				}
				return ok__()
			}
		}
		ret.pop_end__();
		ok__()
	}
}

impl code_::Item_ for Args_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.super_.kw__()}
	fn s__(&self, ret:&mut String, w:&World_) {
		let mut s = String::new();
		s.push_str(ARGS_);
		if self.begin_ != 1 {
			w.rem__(&self.begin_.to_string(), &mut s);
		}
		if self.end_ != core::i32::MAX {
			w.rem__(&self.end_.to_string(), &mut s);
		}
		self.super_.s__(&s, ret, w)
	}
	fn hello__(&self, env:&code_::Env_, _wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		match self.super_.super_.qv4rem__(&self.super_.rems_, |_| {false}, env.q.clone(), env.w.clone()) {
			Ok(q2) =>
				Self::hello__(false, q2.unwrap(), self.begin_, self.end_, ret),
			Err(e) =>
				e,
		}
	}
}
