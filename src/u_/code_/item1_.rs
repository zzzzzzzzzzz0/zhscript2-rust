use super::{*, };

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
			env:&Env_, wm:&mut WorldMut_, ret2: &mut result_::List_) -> Result2_ {
		let a = t_::o__(&a);
		let mut ret4 = result_::List_::new();
		let mut idx = 0;
		a.hello2__(&mut idx, sp, env, wm, &mut ret4)?;
		for i in ret4.iter() {
			let i = as_ref__!(i);
			i.s__(s);
			for rem in &i.rem_ {
				frem(rem)?;
			}
		}
		idx = sp + 1;
		a.hello2__(&mut idx, core::usize::MAX, env, wm, ret2)
	}
	
	pub fn qv4rem_1__(&self, rem:&str, mut shou:impl FnMut(&str) -> bool, q:qv_::T_, w:world_::T_)
			-> Result<Option<qv_::T_>, Result2_> {
		let mut q2 = Some(q);
		match rem {
			"顶" =>
				q2 = Some(as_ref__!(w).top_q_.clone()),
			"上" => {
				q2 = match &as_ref__!(q2.unwrap()).up_ {
					Some(q3) => Some(q3.clone()),
					None => return Err(result2_::err2__("无上")),
				}
			}
			_ => {
				let ret2 = qv_::for__(q2.clone().unwrap(), w.clone(), |q3, _, _| {
					if as_ref__!(q3).name_.contains(&rem.to_string()) {
						q2 = Some(q3.clone());
						Some(())
					} else {None}
				});
				if ret2.is_none() && !shou(rem) {
					return Err(result2_::err__([&as_ref__!(w).text__(rem), "注解不支持"].concat()))
				}
			}
		}
		Ok(q2)
	}
	pub fn qv4rem__(&self, rems:&[String], mut shou:impl FnMut(&str) -> bool, q:qv_::T_, w:world_::T_)
			-> Result<Option<qv_::T_>, Result2_> {
		let mut q2 = Some(q);
		for rem in rems {
			match self.qv4rem_1__(rem.as_str(), &mut shou, q2.clone().unwrap(), w.clone()) {
				Ok(q3) => q2 = q3,
				e => return e
			}
		}
		Ok(q2)
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
	fn hello__(&self, _env:&Env_, _wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		if let keyword_::Id_::Brkpoint = &self.kw_.id_ {
			#[allow(non_snake_case)]
			#[allow(unused_variables)]
			let o_X_o = true;
		} else {
			ret.add2__(self.kw_.clone());
		}
		ok__()
	}
}
