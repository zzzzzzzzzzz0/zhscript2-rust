use super::{u_::*};

pub struct Item_ {
	kw_:keyword_::RI_,
	pub codes_:code_::OL_,
	pub rems_:code_::List_,
	pub cnt_:Cnt_,
	break_:i32,
	continue_:i32,
	loop_:bool,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_, break_:i32) -> Self {
		Self {kw_:match break_ {
			jump_::BREAK_ => kws.for_.clone(),
			jump_::BREAK2_ => kws.range_.clone(),
			_ => panic!()
		}, codes_:None, rems_:code_::List_::new(),
			cnt_:Cnt_::new(match break_ {
				jump_::BREAK2_ => true,
				_ => false
			}), break_,
			continue_:match break_ {
				jump_::BREAK2_ => jump_::CONTINUE2_,
				_ => jump_::CONTINUE_,
			}, loop_:match break_ {
				jump_::BREAK2_ => false,
				_ => true
			}
		}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {&self.kw_}
	fn add__(&mut self, a:code_::List_) -> Result2_ {Item1_::add__(self, a)}
	fn a__(&self) -> code_::ORL_ {Item1_::a__(self)}
	fn a2__(&self) -> code_::ORL_ {Item1_::a2__(self)}
	fn s__(&self, ret:&mut String, w:&World_) {Item1_::s__(self, ret, w)}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		Item1_::hello__(self, env)
	}
}

impl Item1_ for Item_ {
	fn cnt2__(&mut self) ->  &mut Cnt_ {&mut self.cnt_}
	fn codes2__(&mut self, a:code_::OL_) {self.codes_ = a}
	fn rems_push__(&mut self, i:code_::I_) {self.rems_.push(i)}
	fn cnt__(&self) ->  &Cnt_ {&self.cnt_}
	fn rems__(&self) -> &code_::List_ {&self.rems_}
	fn codes__(&self) -> &code_::OL_ {&self.codes_}
	fn break__(&self) -> i32 {self.break_}
	fn continue__(&self) -> i32 {self.continue_}
	fn loop__(&self) -> bool {self.loop_}
}

#[derive(Default, Clone)]
pub struct Cnt_ {
	max_:Option<i64>,
	start_:Option<i64>,
	name_:Option<String>,
	no_:bool,
}

impl Cnt_ {
	pub fn new(no_:bool) -> Self {
		Self {no_, ..Default::default()}
	}
	
	pub fn max2__(&mut self, i:i64) {
		self.max_ = Some(i);
	}
	pub fn start2__(&mut self, i:i64) {
		self.start_ = Some(i);
	}
	pub fn name2__(&mut self, s:&str) -> Result2_ {
		if s.is_empty() {
			return result2_::err2__("名非法")
		}
		self.name_ = Some(s.to_string());
		ok__()
	}
	
	fn hello__(&mut self, rem:&str, idx:usize, has:&mut bool) -> Result2_ {
		if !self.no_ {
			if idx == 0 && self.max_.is_none() {
				if let Some(i2) = t_::s2n__(rem) {
					self.max2__(i2);
					*has = true;
					return ok__()
				}
				if rem.is_empty() {
					self.max2__(core::i64::MAX);
					*has = true;
					return ok__()
				}
			}
			if self.start_.is_none() {
				if let Some(i2) = t_::s2n__(rem) {
					self.start2__(i2);
					*has = true;
					return ok__()
				}
			}
		}
		if self.name_.is_none() {
			self.name2__(rem)?;
			*has = true;
		}
		ok__()
	}
}

pub trait Item1_ : code_::Item_ {
	fn cnt2__(&mut self) ->  &mut Cnt_;
	fn codes2__(&mut self, a:code_::OL_);
	fn rems_push__(&mut self, i:code_::I_);
	fn cnt__(&self) ->  &Cnt_;
	fn rems__(&self) -> &code_::List_;
	fn codes__(&self) -> &code_::OL_;
	
	fn add__(&mut self, mut a:code_::List_) -> Result2_ {
		{
			{
				let mut rem = String::new();
				let cnt2 = self.cnt2__();
				let mut v = vec![];
				for (idx, i) in a.iter().enumerate() {
					let (has, has2) = super::rem2_::text__(i, &mut rem);
					if has && has2 {
						let mut has3 = false;
						cnt2.hello__(&rem, idx, &mut has3)?;
						if has3 {
							v.push(idx);
							continue
						}
						break
					}
				}
				v.reverse();
				for i in v {
					a.remove(i);
				}
			}
			{
				let mut idx = 0;
				while idx < a.len() {
					if as_ref__!(a[idx]).kw__().id_ == keyword_::Id_::BeginRem2 {
						self.rems_push__(a.remove(idx));
						continue
					}
					idx += 1
				}
			}
		}
		if !a.is_empty() {
			self.codes2__(Some(a));
		}
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {Some(self.rems__())}
	fn a2__(&self) -> code_::ORL_ {t_::some__(self.codes__())}
	fn s__(&self, ret:&mut String, w:&World_) {
		self.s_kw__(ret);
		if let Some(i) = self.cnt__().max_ {
			w.rem__(&i.to_string(), ret);
		}
		if let Some(i) = self.cnt__().start_ {
			w.rem__(&i.to_string(), ret);
		}
		if let Some(name) = &self.cnt__().name_ {
			w.rem__(name, ret);
		}
	}

	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		let mut cnt = self.cnt__().clone();
		code_::rems__(self.rems__(), |i, idx, has| {cnt.hello__(i, idx, has)}, env)?;
		let (max, use_i) = match cnt.max_ {
			Some(i) => (if i < 0 {0} else {i as usize}, true),
			None => (core::usize::MAX - 1, cnt.name_.is_some())
		};
		let mut i = 0;
		loop {
			if use_i {
				i += 1;
				if i > max {
					break
				}
			}
			if let Some(name) = &cnt.name_ {
				let i = if let Some(i2) = cnt.start_ {
					if i2 < 0 {
						(i as i64 + i2).to_string()
					} else {
						(i + i2 as usize).to_string()
					}
				} else {
					i.to_string()
				};
				qv_::val__(name, &i, env.q.clone(), env.w.clone());
			}
			if let Some(codes) = self.codes__() {
				let mut act = 0;
				let mut ok__ = |i| {
					act = if i == self.break__() {
						jump_::BREAK_
					} else {
						jump_::CONTINUE_
					};
					ok__()
				};
				result2_::item__(codes.hello__(env), |ret| {
					if let Err((i, _, s, _)) = &ret {
						if *i == self.break__() || *i == self.continue__() {
							if s.is_empty() {
								return ok__(*i)
							}
							if let Some(name) = &cnt.name_ {
								#[cfg(debug_assertions)]
								if as_ref__!(env.w).dbg_.lc_ {
									lc3__!("({}={})", s, name);
								}
								if s == name {
									return ok__(*i)
								}
							}
						}
					}
					ret
				})?;
				if act == jump_::BREAK_ {
					break
				}
				//因2不可省
				if act == jump_::CONTINUE_ {
					continue
				}
			}
			if !self.loop__() {
				break
			}
		}
		ok__()
	}
	
	fn break__(&self) -> i32;
	fn continue__(&self) -> i32;
	fn loop__(&self) -> bool;
}
