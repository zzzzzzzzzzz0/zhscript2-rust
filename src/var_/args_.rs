use super::{*, super::{as_ref__}};

pub struct Args_ {
	super_:Item1_,
	begin_:i32,
	end_:i32,
	line_:bool,
}

impl Args_ {
	pub fn new(kws:&keyword_::List_, rems:Vec<String>, begin_:i32, end_:i32, line_:bool) -> Self {
		Self {super_:Item1_::new(&kws, rems), begin_, end_, line_}
	}
	pub fn with__(name:&str, rems:&mut Vec<String>) -> Option<(i32, i32, bool)> {
		let mut begin = 1;
		let mut end = core::i32::MAX;
		let mut begin2 = core::usize::MAX;
		let mut end2 = core::usize::MAX;
		let b1 = name == ARGS_;
		let b2 = name == ARG_;
		if b1 || b2 {
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
			Some((begin, end, b2))
		} else {
			None
		}
	}
	pub fn with2__(name:&str, rems:&Vec<String>) -> bool {
		if name == ARGS_ || name == ARG_ {
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

	pub fn hello__(is_has:bool, q:qv_::T_, begin:i32, end:i32, line:bool, ret:&mut result_::List_) -> Result2_ {
		{
			let q = as_ref__!(q);
			let ls = &as_ref__!(q.args_);
			let a = &ls.a_;
			if !a.is_empty() {
				let begin2 = t_::i2u__(begin, ls.len__());
				let end2 = t_::i2u__(end, ls.len__());
				if end2 >= begin2 {
					if is_has {
						ret.add__("1");
						return ok__()
					}
					if line {
						if begin2 == 1 && end2 >= a.len() {
						} else {
							let mut s = String::new();
							let mut s2 = String::new();
							let mut i2 = 1;
							let mut b = false;
							let s2s = |s2:&mut String, b:&mut bool, s:&mut String| {
								if *b {
									result_::List_::s2s__(s2, b'1', s);
									*b = false;
								}
							};
							for i in a {
								let i = as_ref__!(i);
								if i.dunhao__() {
									i2 += 1;
									s2s(&mut s2, &mut b, &mut s);
									continue
								}
								if i2 < begin2 {
									continue
								}
								if i2 > end2 {
									break
								}
								i.s__(&mut s2);
								b = true;
							}
							s2s(&mut s2, &mut b, &mut s);
							ret.add__(s);
							return ok__()
						}
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
						return ok__()
					}
				}
			}
		}
		if line {
			ret.add__(as_mut_ref__!(q).args_1__());
			return ok__()
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
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		match qv_::rem4_::hello__(&self.super_.rems_, |_| {false}, env.in_q_.clone(), env.q.clone(), env.w.clone()) {
			Ok(q2) =>
				Self::hello__(false, q2.unwrap(), self.begin_, self.end_, self.line_, &mut as_mut_ref__!(env.ret)),
			Err(e) =>
				e,
		}
	}
}
