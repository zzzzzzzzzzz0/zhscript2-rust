use super::{u_::*, *};

pub const NO_ARG_:&str = "无参";
pub const BACK_ARG_:&str = "倒挂";

pub struct Item_ {
	super_:set_::Item_,
	sp_:usize,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:set_::Item_::new2(&kws.def_, &kws.equ_), sp_:code_::split2_0__()}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn kw2__(&self) -> keyword_::ORI_ {self.super_.kw2__()}

	fn add__(&mut self, a:code_::List_) -> Result2_ {
		self.super_.super_.chk_empty__(&a, "名缺")?;
		code_::split2_1__(&a, &mut self.sp_);
		self.super_.add__(a)
	}
	fn add2__(&mut self, a:code_::List_) -> Result2_ {self.super_.add2__(a)}
	fn a__(&self) -> code_::ORL_ {self.super_.a__()}
	fn a2__(&self) -> code_::ORL_ {self.super_.a2__()}

	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		let mut name = String::new();
		let mut argc = core::usize::MAX;
		let mut backarg = 0;
		let ret2 = t__(result_::List_::new());
		let mut q2 = Some(env.q.clone());
		code_::split2_2__(self.a__(), self.sp_, &mut name, |rem| {
			match rem {
				NO_ARG_ => argc = 0,
				_ => {
					if rem.starts_with(BACK_ARG_) {
						let s = &rem[BACK_ARG_.len()..];
						backarg = if s.is_empty() {1} else {
							match s.parse::<usize>() {
								Ok(i) => i,
								Err(e) => return result2_::err__(e.to_string())
							}
						};
						return ok__()
					}
					match qv_::rem4_::hello1__(rem, |_| {
						false
					}, q2.clone().unwrap(), env.w.clone()) {
						Ok(q3) => q2 = q3,
						Err(e) => return e,
					}
				}
			}
			ok__()
		}, &code_::Env_::new7(code_::Opt_ {undef_eq_text_:true, ..env.gd}, ret2.clone(), env))?;
		
		if name.is_empty() {return self.super_.super_.err__("名空")}
		{
			let cs:Vec<char> = name.chars().collect();
			let paichu = vec![];
			if let Some(o) = qv_::for__(env.q.clone(), env.w.clone(), |q, _, _| {
				as_ref__!(q).defs_.find__(&cs, 0, false, false, &paichu)
			}) {
				let w = as_ref__!(env.w);
				return self.super_.super_.err__(&["名", &w.text__(&name), "被",
					&w.text__(as_ref__!(o.2).name__()), "覆盖"].concat())
			}
		}
		
		let ret3 = t__(result_::List_::new());
		t_::o__(&self.a2__()).hello__(&code_::Env_::new6(ret3.clone(), env))?;

		let ret2 = as_ref__!(ret2);
		let ret3 = as_ref__!(ret3);
		if !ret2.is_empty() {
			argc = ret2.len__()
		}
		qv_::def__(&name, &ret3.s__(), argc, backarg, Some(&ret2), q2.unwrap(), env.w.clone())
	}
}
