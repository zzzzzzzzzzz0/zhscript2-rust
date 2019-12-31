use super::u_::*;
	
pub fn text__(i:&code_::BI_, ret:&mut String) -> (bool, bool) {
	if i.kw__().id_ == keyword_::Id_::BeginRem2 {
		ret.clear();
		if let Some(a) = &i.a__() {
			for i2 in a.iter() {
				match i2.kw__().id_ {
					keyword_::Id_::BeginText => {
						ret.push_str(i2.s2__())
					}
					_ => return (true, false),
				}
			}
		} else {
			ret.push_str(i.s2__())
		}
		(true, true)
	} else {
		(false, false)
	}
}

pub fn new(kws:&keyword_::List_, codes:&code_::List_) -> code_::BI_ {
	let i1 = code_::Item1_::new2(&kws.begin_rem2_, &kws.end_rem2_);
	if codes.len() == 1 {
		let i = &codes[0];
		if i.kw__().id_ == keyword_::Id_::BeginText {
			return Box::new(SimpItem_ {super_:i1, s_:i.s2__().to_string()})
		}
	}
	Box::new(Item_ {super_:i1, a_:None})
}

struct Item_ {
	super_:code_::Item1_,
	a_:code_::OL_,
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn kw2__(&self) -> keyword_::ORI_ {self.super_.kw2__()}
	fn add__(&mut self, a:code_::List_) -> Result2_ {
		self.a_ = Some(a);
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}
	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		Item1_::hello__(self, gd, q, w, ret)
	}
}

impl Item1_ for Item_ {
	fn hello2__(&self, gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		t_::o__(&self.a_).hello__(gd, q, w, ret)
	}
}

struct SimpItem_ {
	super_:code_::Item1_,
	s_:String,
}

impl code_::Item_ for SimpItem_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn s__(&self, ret:&mut String, _w:&World_) {self.super_.s2__(&self.s_, ret)}
	fn s2__(&self) -> &str {&self.s_}
	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		Item1_::hello__(self, gd, q, w, ret)
	}
}

impl Item1_ for SimpItem_ {
	fn hello2__(&self, _:code_::Opt_, _:qv_::T_, _:&mut World_, ret:&mut result_::List_) -> Result2_ {
		ret.add__(self.s_.to_string());
		ok__()
	}
}

trait Item1_ : code_::Item_ {
	fn hello2__(&self, gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_;

	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		let mut ret2 = result_::List_::new();
		self.hello2__(gd, q, w, &mut ret2)?;
		if w.dbg_.lc_ {
			if let Some(kw) = self.kw2__() {
				w.dbg_.lc_kw__(&kw);
			}
		}
		if let Some(i2) = ret.end__() {
			for i in ret2.iter() {
				let mut s = String::new();
				i.borrow().s__(&mut s);
				i2.borrow_mut().rem_.push(s.to_string())
			}
			ok__()
		} else {
			result2_::err2__("无法注解")
		}
	}
}
