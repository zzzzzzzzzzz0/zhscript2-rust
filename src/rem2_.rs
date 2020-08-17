use super::u_::*;
	
pub fn text__(i:&code_::I_, ret:&mut String) -> (bool, bool) {
	if as_ref__!(i).kw__().id_ == keyword_::Id_::BeginRem2 {
		ret.clear();
		if let Some(a) = &as_ref__!(i).a__() {
			for i2 in a.iter() {
				match as_ref__!(i2).kw__().id_ {
					keyword_::Id_::BeginText => {
						ret.push_str(as_ref__!(i2).s2__())
					}
					_ => return (true, false),
				}
			}
		} else {
			ret.push_str(as_ref__!(i).s2__())
		}
		(true, true)
	} else {
		(false, false)
	}
}

pub fn new(kws:&keyword_::List_, codes:&code_::List_) -> code_::OI_ {
	let super_ = code_::Item1_::new2(&kws.begin_rem2_, &kws.end_rem2_);
	if codes.len() == 1 {
		let i = &codes[0];
		if as_ref__!(i).kw__().id_ == keyword_::Id_::BeginText {
			return code_::oi__(SimpItem_ {super_, s_:as_ref__!(i).s2__().to_string()})
		}
	}
	code_::oi__(Item_ {super_, a_:None})
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
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		Item1_::hello__(self, env)
	}
}

impl Item1_ for Item_ {
	fn hello2__(&self, env:&code_::Env_) -> Result2_ {
		t_::o__(&self.a_).hello__(env)
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
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		Item1_::hello__(self, env)
	}
}

impl Item1_ for SimpItem_ {
	fn hello2__(&self, env:&code_::Env_) -> Result2_ {
		as_mut_ref__!(env.ret).add__(self.s_.to_string());
		ok__()
	}
}

trait Item1_ : code_::Item_ {
	fn hello2__(&self, env:&code_::Env_) -> Result2_;

	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		let ret2 = t__(result_::List_::new());
		self.hello2__(&code_::Env_::new6(ret2.clone(), env))?;
		if as_ref__!(env.w).dbg_.lc_ {
			if let Some(kw) = self.kw2__() {
				as_ref__!(env.w).dbg_.lc_kw__(&kw);
			}
		}
		let ret2 = as_ref__!(ret2);
		if let Some(i2) = as_mut_ref__!(env.ret).end__() {
			for i in ret2.iter() {
				let mut s = String::new();
				as_ref__!(i).s__(&mut s);
				as_mut_ref__!(i2).rem_.push(s.to_string())
			}
			ok__()
		} else {
			result2_::err2__("无法注解")
		}
	}
}
