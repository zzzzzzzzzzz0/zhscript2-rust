use super::u_::*;

pub struct Item_ {
	super_:code_::Item1_,
	a_:code_::OL_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:code_::Item1_::new(&kws.guandaojie2_), a_:None}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn add__(&mut self, a:code_::List_) -> Result2_ {
		self.super_.chk_empty__(&a, "缺")?;
		self.a_ = Some(a);
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		let ret2 = t_::o__(&self.a_).hello__(env);
		let mut ret = as_mut_ref__!(env.ret);
		as_ref__!(env.w).dunhao__(&mut ret);
		if let Err((i, i2, s, s2)) = ret2 {
			ret.add__(i);
			as_ref__!(env.w).dunhao__(&mut ret);
			ret.add__(i2);
			as_ref__!(env.w).dunhao__(&mut ret);
			ret.add__(s);
			as_ref__!(env.w).dunhao__(&mut ret);
			ret.add__(s2);
		}
		ok__()
	}
}
