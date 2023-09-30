use super::{u_::*, rem2_};

pub struct ItemA_ {
	pub super_:code_::Item1_,
	pub a_:code_::OL_,
	sp_:usize, sp_in_end_:bool,
	in_cur_qv_:bool,
}

impl ItemA_ {
	pub fn new(kw: &keyword_::RI_) -> Self {
		Self {super_:code_::Item1_::new(kw), a_:None, sp_:code_::split2_0__(), sp_in_end_:false, in_cur_qv_:false}
	}
}

pub struct Item_ {
	super_:ItemA_,
	arg0_:String, i_arg0_:code_::OI_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:ItemA_::new(&kws.eval_), arg0_:String::new(), i_arg0_:None}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.super_.kw__()}
	fn add__(&mut self, a:code_::List_) -> Result2_ {Item1_::add__(self, a)}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.super_.a_)}
	fn s__(&self, ret:&mut String, w:&World_) {Item1_::s__(self, ret, w)}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		Item1_::hello__(self, env)
	}
}

impl Item1_ for Item_ {
	fn super__(&self) -> &ItemA_ {&self.super_}
	fn super_mut__(&mut self) -> &mut ItemA_ {&mut self.super_}
	fn arg0__(&self) -> &str {&self.arg0_}
	fn arg02__(&mut self, i:code_::I_) -> Result2_ {
		{
			let mut ret = String::new();
			let (b, b2) = rem2_::text__(&i, &mut ret);
			if b && b2 {
				self.arg0_ = ret;
			}
		}
		self.i_arg0_ = Some(i);
		ok__()
	}
	fn i_arg0__(&self) -> code_::OI_ {self.i_arg0_.clone()}
}

pub trait Item1_ : code_::Item_ {
	fn super__(&self) -> &ItemA_;
	fn super_mut__(&mut self) -> &mut ItemA_;
	fn src__(&self, s:String, src2:&mut String, _:&mut bool, _:qv_::T_, _:world_::T_) -> Result2_ {
		*src2 = s;
		ok__()
	}
	fn arg0__(&self) -> &str {""}
	fn arg02__(&mut self, _:code_::I_) -> Result2_ {result2_::err2__("不能设参数0")}
	fn i_arg0__(&self) -> code_::OI_ {None}

	fn add__(&mut self, mut a:code_::List_) -> Result2_ {
		let mut in_cur_qv = false;
		{
			let mut idx2 = vec![];
			let mut idx = 0;
			while idx < a.len() {
				let i = &a[idx];
				if as_ref__!(i).kw__().id_ == keyword_::Id_::BeginRem2 {
					{
						let mut ret = String::new();
						let (b, b2) = rem2_::text__(i, &mut ret);
						if b && b2 {
							if
								match ret.as_str() {
									"在当前区" => {
										in_cur_qv = true;
										true
									}
									_ => {false}
								}
							{
								idx2.push(idx);
								idx += 1;
								continue
							}
						}
					}
					self.arg02__(a.remove(idx))?;
					break
				}
				idx += 1
			}
			for idx in idx2.iter().rev() {
				a.remove(*idx);
			}
		}
		if a.is_empty() {
			return result2_::qve__();
		}
		let mut sp = code_::split2_0__();
		code_::split2_1__(&a, &mut sp);
		self.super_mut__().sp_ = sp;
		self.super_mut__().sp_in_end_ = sp == a.len() - 1;
		self.super_mut__().a_ = Some(a);
		self.super_mut__().in_cur_qv_ = in_cur_qv;
		ok__()
	}

	fn s__(&self, ret:&mut String, w:&World_) {
		ret.push_str(&self.kw__().s_);
		if let Some(i) = self.i_arg0__() {
			let ms = code_::MS_::new(ret);
			code_::List_::s3_i__(&i, &ms, w);
		}
	}

	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		let in_cur_qv = self.super__().in_cur_qv_;
		let q2 = if in_cur_qv {env.q.clone()} else {t__(Qv_::new2(Some(env.q.clone())))};
		let mut src2 = String::new();
		{
			let mut src = String::new();
			let w2 = env.w.clone();
			{
				as_mut_ref__!(q2).args_1clear__();
				let args = &as_ref__!(q2).args_;
				let cl = self.super__().a_.as_ref();
				if in_cur_qv {
					let mut args = as_mut_ref__!(args);
					if !args.is_empty() && cl.unwrap().len() > 1 {
						as_ref__!(env.w).dunhao__(&mut args);
					}
				}
				code_::split2_2__(cl, self.super__().sp_, &mut src,
					|rem| as_ref__!(w2).no_rem2__(&rem),
					&code_::Env_::new6(args.clone(), env))?;
				let mut args = as_mut_ref__!(args);
				if args.is_empty() && self.super__().sp_in_end_ {
					args.add__("")
				}
			}
			if !in_cur_qv {
				as_mut_ref__!(q2).src_ = src.to_string();
			}
			let mut once = false;
			self.src__(src, &mut src2, &mut once, q2.clone(), w2)?;
			if once {
				return ok__()
			}
		}
		if !in_cur_qv {
			let mut arg0 = self.arg0__().to_string();
			if arg0.is_empty() {
				if let Some(i) = self.i_arg0__() {
					let i = as_ref__!(i);
					if let Some(a) = i.a__() {
						let ret2 = t__(result_::List_::new());
						a.hello__(&code_::Env_::new6(ret2.clone(), env))?;
						arg0 = as_ref__!(ret2).s__();
					}
				}
			}
			if !arg0.is_empty() {
				as_mut_ref__!(q2).arg0_ = arg0;
			}
		}
		eval_::hello__(&src2, &code_::Env_::new2(q2, env))
	}
}
