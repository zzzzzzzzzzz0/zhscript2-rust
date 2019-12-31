use super::u_::*;
use super::rem2_;

pub struct Item_ {
	kw_:keyword_::RI_,
	pub codes_:code_::OL_,
	pub rems_:code_::List_,
	pub cnt_:Cnt_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {kw_:kws.for_.clone(), codes_:None, rems_:code_::List_::new(), cnt_:Cnt_::new()}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {&self.kw_}
	fn add__(&mut self, a:code_::List_) -> Result2_ {Item1_::add__(self, a)}
	fn a__(&self) -> code_::ORL_ {Item1_::a__(self)}
	fn a2__(&self) -> code_::ORL_ {Item1_::a2__(self)}
	fn s__(&self, ret:&mut String, w:&World_) {Item1_::s__(self, ret, w)}
	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		Item1_::hello__(self, gd, q, w, ret)
	}
}

impl Item1_ for Item_ {
	fn cnt2__(&mut self) ->  &mut Cnt_ {&mut self.cnt_}
	fn codes2__(&mut self, a:code_::OL_) {self.codes_ = a}
	fn rems_push__(&mut self, i:code_::BI_) {self.rems_.push(i)}
	fn cnt__(&self) ->  &Cnt_ {&self.cnt_}
	fn rems__(&self) -> &code_::List_ {&self.rems_}
	fn codes__(&self) -> &code_::OL_ {&self.codes_}
	fn break__(&self) -> i32 {jump_::BREAK_}
	fn continue__(&self) -> i32 {jump_::CONTINUE_}
	fn loop__(&self) -> bool {true}
}

pub struct Cnt_ {
	max_:usize,
	name_:Option<String>,
	max2_:usize,
	name2_:usize,
	i_:usize,
}

impl Cnt_ {
	const NO_:usize = core::usize::MAX - 1;
	pub fn new() -> Self {
		Self::new2(core::usize::MAX, None, core::usize::MAX, core::usize::MAX)
	}
	fn new2(i:usize, name:Option<String>, i2:usize, name2:usize) -> Self {
		Self {max_:i, name_:name, max2_:i2, name2_:name2, i_:0}
	}
	
	#[allow(dead_code)]
	pub fn max2__(&mut self, i:usize) {
		self.max3__(i, Self::NO_)
	}
	fn max3__(&mut self, i:usize, idx:usize) {
		self.max2_ = idx;
		self.max_ = i;
	}
	#[allow(dead_code)]
	pub fn name2__(&mut self, s:&str) -> Result2_ {
		self.name3__(s, Self::NO_)
	}
	pub fn name3__(&mut self, s:&str, idx:usize) -> Result2_ {
		self.name2_ = idx;
		if s.is_empty() {
			return result2_::err2__("名非法")
		}
		self.name_ = Some(s.to_string());
		ok__()
	}
	
	fn hello__(&mut self, rem:&str, idx:usize, has:&mut bool) -> Result2_ {
		if self.max2_ == core::usize::MAX {
			if let Some(i2) = t_::s2n__(rem) {
				self.max3__(i2, idx);
				*has = true;
				return ok__()
			}
			//self.max2_ = Self::NO_;
		}
		if self.name2_ == core::usize::MAX {
			self.name3__(rem, idx)?;
			*has = true;
		}
		ok__()
	}
	fn next__(&mut self) -> bool {
		self.i_ += 1;
		self.i_ <= self.max_
	}
}

pub trait Item1_ : code_::Item_ {
	fn cnt2__(&mut self) ->  &mut Cnt_;
	fn codes2__(&mut self, a:code_::OL_);
	fn rems_push__(&mut self, i:code_::BI_);
	fn cnt__(&self) ->  &Cnt_;
	fn rems__(&self) -> &code_::List_;
	fn codes__(&self) -> &code_::OL_;
	
	fn add__(&mut self, mut a:code_::List_) -> Result2_ {
		{
			{
				let mut rem = String::new();
				let cnt2 = self.cnt2__();
				for (idx, i) in a.iter().enumerate() {
					let (has, has2) = rem2_::text__(&i, &mut rem);
					if has && has2 {
						let mut has3 = false;
						cnt2.hello__(&rem, idx, &mut has3)?;
						if has3 {
							continue
						}
						break
					}
				}
				if cnt2.name2_ < Cnt_::NO_ {
					a.remove(cnt2.name2_);
				}
				if cnt2.max2_ < Cnt_::NO_ {
					a.remove(cnt2.max2_);
				}
			}
			{
				let mut idx = 0;
				while idx < a.len() {
					if a[idx].kw__().id_ == keyword_::Id_::BeginRem2 /*|| idx < a.len() - 1*/ {
						self.rems_push__(a.remove(idx));
						continue
					}
					idx += 1
				}
			}
		}
		if a.is_empty() {
			return result2_::qve__();
		}
		self.codes2__(Some(a));
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {Some(&&self.rems__())}
	fn a2__(&self) -> code_::ORL_ {t_::some__(&&self.codes__())}
	fn s__(&self, ret:&mut String, w:&World_) {
		ret.push_str(&self.kw__().s_);
		if self.cnt__().max_ != core::usize::MAX {
			w.rem__(&self.cnt__().max_.to_string(), ret);
		}
		if let Some(name) = &self.cnt__().name_ {
			w.rem__(&name, ret);
		}
	}

	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		let mut cnt = Cnt_::new2(self.cnt__().max_, self.cnt__().name_.clone(),
								self.cnt__().max2_, self.cnt__().name2_);
		{
			let mut ret3 = result_::List_::new();
			ret3.add2__(w.kws_.begin_text_.clone());
			self.rems__().hello__(gd, q.clone(), w, &mut ret3)?;
			for i in &ret3.end__().unwrap().borrow().rem_ {
				let mut has = false;
				cnt.hello__(i, 0, &mut has)?;
				if has {
					continue
				}
			}
		}
		while cnt.next__() {
			if let Some(name) = &cnt.name_ {
				qv_::val__(&name, &cnt.i_.to_string(), q.clone(), w);
			}
			let mut act = 0;
			let mut ok__ = |i| {
				act = if i == self.break__() {
					jump_::BREAK_
				} else {
					jump_::CONTINUE_
				};
				ok__()
			};
			result2_::item__(t_::o__(self.codes__()).hello__(gd, q.clone(), w, ret), |ret| {
				if let Err((i, s, _)) = &ret {
					if *i == self.break__() || *i == self.continue__() {
						if s.is_empty() {
							return ok__(*i)
						}
						if let Some(name) = &cnt.name_ {
							if w.dbg_.lc_ {
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
