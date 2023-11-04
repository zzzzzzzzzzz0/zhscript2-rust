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

fn qv4rem_z__(rem:&str, is_has:bool, mut shou:impl FnMut(&str) -> bool, in_q:Option<qv_::T_>, q:qv_::T_, w:world_::T_)
		-> Result<Option<qv_::T_>, Result2_> {
	let mut q2 = Some(q);
	match rem {
		qv_::TOP_ =>
			q2 = Some(as_ref__!(w).top_q_.clone()),
		qv_::UP_ => {
			q2 = match &as_ref__!(q2.unwrap()).up_ {
				Some(q3) => Some(q3.clone()),
				None => {
					if is_has {None} else {
						return Err(result2_::err2__("无上"))
					}
				}
			}
		}
		qv_::IN_ =>
			if let Some(q3) = in_q {
				q2 = Some(q3.clone())
			} else {
				return Err(as_ref__!(w).no_rem2__(rem))
			}
		_ => {
			let ret2 = qv_::for__(q2.clone().unwrap(), w.clone(), |q3, _, _| {
				if as_ref__!(q3).name_.contains(&rem.to_string()) {
					q2 = Some(q3);
					Some(())
				} else {None}
			});
			if ret2.is_none() {
				for q3 in &as_ref__!(w).datas_ {
					if as_ref__!(q3).name_.contains(&rem.to_string()) {
						return Ok(Some(q3.clone()))
					}
				}
			}
			if ret2.is_none() && !shou(rem) {
				return Err(as_ref__!(w).no_rem2__(rem))
			}
		}
	}
	Ok(q2)
}
pub fn qv4rem_2__(rems:&[String], is_has:bool, shou:impl FnMut(&str) -> bool, q:qv_::T_, w:world_::T_)
		-> Result<Option<qv_::T_>, Result2_> {
	qv4rem_4__(rems, is_has, shou, None, q, w)
}
pub fn qv4rem_5__(rems:&[String], shou:impl FnMut(&str) -> bool, in_q:Option<qv_::T_>, q:qv_::T_, w:world_::T_)
		-> Result<Option<qv_::T_>, Result2_> {
	qv4rem_4__(rems, false, shou, in_q, q, w)
}
pub fn qv4rem_4__(rems:&[String], is_has:bool, mut shou:impl FnMut(&str) -> bool, in_q:Option<qv_::T_>, q:qv_::T_, w:world_::T_)
		-> Result<Option<qv_::T_>, Result2_> {
			let mut q2 = Some(q);
	for rem in rems {
		match qv4rem_z__(rem.as_str(), is_has, &mut shou, in_q.clone(), q2.clone().unwrap(), w.clone()) {
			Ok(q3) => q2 = q3,
			e => return e
		}
	}
	Ok(q2)
}
pub fn qv4rem_1__(rem:&str, shou:impl FnMut(&str) -> bool, q:qv_::T_, w:world_::T_)
		-> Result<Option<qv_::T_>, Result2_> {
	qv4rem_z__(rem, false, shou, None, q, w)
}
#[allow(dead_code)]
pub fn qv4rem__(rems:&[String], shou:impl FnMut(&str) -> bool, q:qv_::T_, w:world_::T_)
		-> Result<Option<qv_::T_>, Result2_> {
	qv4rem_2__(rems, false, shou, q, w)
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
