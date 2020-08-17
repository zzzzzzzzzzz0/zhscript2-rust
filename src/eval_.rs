use super::u_::*;

pub struct Item_ {
	super_:code_::Item1_,
	a_:code_::OL_,
	sp_:usize,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:code_::Item1_::new(&kws.eval_), a_:None, sp_:code_::Item1_::split2_0__()}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn add__(&mut self, a:code_::List_) -> Result2_ {Item1_::add__(self, a)}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		Item1_::hello__(self, env)
	}
}

impl Item1_ for Item_ {
	fn codes2__(&mut self, a:code_::OL_) {self.a_ = a}
	fn codes__(&self) -> &code_::OL_ {&self.a_}
	fn sp2__(&mut self, i:usize) {self.sp_ = i}
	fn sp__(&self) -> usize {self.sp_}
}

pub trait Item1_ : code_::Item_ {
	fn codes2__(&mut self, a:code_::OL_);
	fn codes__(&self) -> &code_::OL_;
	fn sp2__(&mut self, i:usize);
	fn sp__(&self) -> usize;
	fn src__(&self, s:String, src2:&mut String, _once:&mut bool, _q:qv_::T_, _w:world_::T_) -> Result2_ {
		*src2 = s;
		ok__()
	}

	fn add__(&mut self, a:code_::List_) -> Result2_ {
		if a.is_empty() {
			return result2_::qve__();
		}
		let mut sp = code_::Item1_::split2_0__();
		code_::Item1_::split2_1__(&a, &mut sp);
		self.sp2__(sp);
		self.codes2__(Some(a));
		ok__()
	}

	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		let q2 = t__(Qv_::new2(Some(env.q.clone())));
		let gd = code_::Opt_ {guandao_du_:env.gd.guandao_du_, guandao_jie_:env.gd.guandao_jie_, ..Default::default()};
		let mut src2 = String::new();
		{
			let mut src = String::new();
			let w2 = env.w.clone();
			code_::Item1_::split2_2__(self.codes__().as_ref(), self.sp__(), &mut src,
				|rem| as_ref__!(w2).no_rem2__(&rem),
				&code_::Env_::new7(gd, as_mut_ref__!(q2).args_.clone(), env))?;
			as_mut_ref__!(q2).src_ = src.to_string();
			let mut once = false;
			self.src__(src, &mut src2, &mut once, q2.clone(), w2)?;
			if once {
				return ok__()
			}
		}
		eval_::hello__(&src2, &code_::Env_::new5(q2, gd, env))
	}
}
