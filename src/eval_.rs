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
	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		Item1_::hello__(self, gd, q, w, ret)
	}
}

impl Item1_ for Item_ {
	fn codes2__(&mut self, a:code_::OL_) {self.a_ = a}
	fn codes__(&self) -> &code_::OL_ {&self.a_}
	fn sp2__(&mut self, i:usize) {self.sp_ = i}
	fn sp__(&self) -> usize {self.sp_}
	fn src__(&self, s:String, src2:&mut String, _q:&mut Qv_, _w:&mut World_) -> Result2_ {
		*src2 = s;
		ok__()
	}
}

pub trait Item1_ : code_::Item_ {
	fn codes2__(&mut self, a:code_::OL_);
	fn codes__(&self) -> &code_::OL_;
	fn sp2__(&mut self, i:usize);
	fn sp__(&self) -> usize;
	fn src__(&self, s:String, src2:&mut String, q:&mut Qv_, w:&mut World_) -> Result2_;

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

	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		let mut q2 = Qv_::new2(Some(q.clone()));
		let mut src2 = String::new();
		{
			let args = &mut q2.args_;
			let mut src = String::new();
			/*t_::o__(self.codes__()).hello__(gd, q.clone(), w, args)?;
			code_::Item1_::split__(args, |i| {
				i.borrow().s__(&mut src);
				ok__()
			})?;*/
			code_::Item1_::split2_2__(self.codes__().as_ref(), self.sp__(), &mut src, |_| false,
				gd, q.clone(), w, args)?;
			q2.src_ = src.to_string();
			self.src__(src, &mut src2, &mut q2, w)?;
		}
		eval_::hello__(&src2, gd, qv_::t__(q2), q, w, ret)
	}
}
