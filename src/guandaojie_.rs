use super::u_::*;

pub struct Item_ {
	super_:code_::Item1_,
	a_:code_::OL_,
	rems_:code_::OL_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:code_::Item1_::new(&kws.guandaojie_), a_:None, rems_:None}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn add__(&mut self, mut a:code_::List_) -> Result2_ {
		{
			let mut rems = code_::List_::new();
			let mut idx = 0;
			while idx < a.len() {
				if as_ref__!(a[idx]).kw__().id_ == keyword_::Id_::BeginRem2 {
					rems.push(a.remove(idx));
					continue
				}
				idx += 1
			}
			if !rems.is_empty() {
				self.rems_ = Some(rems)
			}
		}
		self.super_.chk_empty__(&a, "缺")?;
		self.a_ = Some(a);
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.rems_)}
	fn a2__(&self) -> code_::ORL_ {t_::some__(&self.a_)}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		let mut s = String::new();
		if let Some(rems) = &self.rems_ {
			code_::rems__(rems, |i, _, has| {
				s.push_str(i);
				*has = true;
				ok__()
			}, env)?;
		}
		if s.is_empty() {
			s.push('1')
		}
		t_::o__(&self.a_).hello__(&code_::Env_::new3(
			code_::Opt_ {guandao_jie_:Box::leak(s.into_boxed_str()), ..env.gd}, env))
	}
}
