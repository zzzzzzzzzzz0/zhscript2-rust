use super::{*};
#[cfg(debug_assertions)]
use super::super::super::{lc3__, lc_kw__, p__, dbp_c__};

pub fn split2_0__() -> usize {core::usize::MAX - 1}
pub fn split2_1__(a:&code_::List_, sp:&mut usize) {
	for (idx, i) in a.iter().enumerate() {
		if as_ref__!(i).kw__().id_ == keyword_::Id_::Dunhao {
			*sp = idx;
			break
		}
	}
}
pub fn split2_2__(a:code_::ORL_, sp:usize, s: &mut String, mut frem:impl FnMut(&str) -> Result2_,
		env:&Env_) -> Result2_ {
	let a = t_::o__(&a);
	let ret4 = t__(result_::List_::new());
	let mut idx = 0;
	a.hello2__(&mut idx, sp, &Env_::new6(ret4.clone(), env))?;
	for i in as_ref__!(ret4).iter() {
		let i = as_ref__!(i);
		i.s__(s);
		for rem in &i.rem_ {
			frem(rem)?;
		}
	}
	idx = sp + 1;
	a.hello2__(&mut idx, core::usize::MAX, env)
}

pub fn rems__(rems:&code_::List_, mut frem:impl FnMut(&str, usize, &mut bool) -> Result2_, env:&code_::Env_) -> Result2_ {
	let mut ret3 = result_::List_::new();
	ret3.add2__(as_ref__!(env.w).kws_.begin_text_.clone());
	let ret3 = t__(ret3);
	rems.hello__(&code_::Env_::new6(ret3.clone(), env))?;
	for (idx, i) in as_ref__!(as_ref__!(ret3).end__().unwrap()).rem_.iter().enumerate() {
		let mut has = false;
		frem(i, idx, &mut has)?;
		if has {
			continue
		}
		return as_ref__!(env.w).no_rem2__(i)
	}
	ok__()
}

#[allow(dead_code)]
pub fn qv4rem__(rems:&[String], shou:impl FnMut(&str) -> bool, q:qv_::T_, w:world_::T_)
		-> Result<Option<qv_::T_>, Result2_> {
	qv_::rem4_::hello2__(rems, false, shou, None, q, w)
}

pub struct Item1_ {
	kw_:keyword_::RI_,
	kw2_:keyword_::ORI_,
	kw3_:keyword_::ORI_,
}

impl Item1_ {
	pub fn new(kw:&keyword_::RI_) -> Self {
		Self {kw_:kw.clone(), kw2_:None, kw3_:None}
	}
	pub fn new2(kw:&keyword_::RI_, kw2:&keyword_::RI_) -> Self {
		Self {kw_:kw.clone(), kw2_:Some(kw2.clone()), kw3_:None}
	}
	pub fn new3(kw:&keyword_::RI_, kw2:&keyword_::RI_, kw3:&keyword_::RI_) -> Self {
		Self {kw_:kw.clone(), kw2_:Some(kw2.clone()), kw3_:Some(kw3.clone())}
	}

	pub fn s2__(&self, s:&str, ret:&mut String) {
		ret.push_str(&self.kw_.s_);
		ret.push_str(s);
		ret.push_str(&t_::or__(&self.kw2_).s_);
	}
	
	pub fn err__(&self, s:&str) -> Result2_ {
		result2_::err__([&self.kw__().s_, s].concat())
	}
	pub fn chk_empty__(&self, a:&List_, s:&str) -> Result2_ {
		if a.a_.is_empty() {self.err__(s)} else {ok__()}
	}
}

impl Item_ for Item1_ {
	fn kw__(&self) -> &keyword_::Item_ {&self.kw_}
	fn kw2__(&self) -> keyword_::ORI_ {self.kw2_.clone()}
	fn kw3__(&self) -> keyword_::ORI_ {self.kw3_.clone()}
	fn hello__(&self, env:&Env_) -> Result2_ {
		if let keyword_::Id_::Brkpoint = &self.kw_.id_ {
			#[cfg(debug_assertions)]
			{
				if dbp_c__!("-var-", env) {
					lc3__!("\nvar {:?}\n", as_ref__!(env.q).vars_.a_);
				}
				{
					let b = dbp_c__!("-ret-", env);
					let b2 = dbp_c__!("-ret2-", env);
					if b || b2 {
						let a = as_ref__!(env.ret);
						lc3__!("\nret");
						if b {
							lc3__!("\t[{}/{}]{:?}\n", a.len(), a.len__(), a);
						}
						if b2 {
							lc3__!("\t{:?}\n", a.to_vec__());
						}
					}
				}
				{
					let b = dbp_c__!("-arg-", env);
					let b2 = dbp_c__!("-arg2-", env);
					if b || b2 {
						let q = as_ref__!(env.q);
						let a = as_ref__!(q.args_);
						lc3__!("\narg");
						if b {
							lc3__!("\t[{}/{}]{:?}\n", a.len(), a.len__(), a);
						}
						if b2 {
							lc3__!("\t{:?}\n", a.to_vec__());
						}
					}
				}
				for i in &["-tree-", "-tree4-", "-set-", "-o-"] {
					if dbp_c__!(i, env) {
						as_mut_ref__!(env.w).dbg_.bp2_.push_str(i);
					}
				}
				#[allow(non_snake_case)]
				#[allow(unused_variables)]
				let o_X_o = true;
			}
		} else {
			as_mut_ref__!(env.ret).add2__(self.kw_.clone());
		}
		ok__()
	}
}
