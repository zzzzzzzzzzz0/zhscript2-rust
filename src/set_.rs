use super::{u_::*, as_ref__};

pub struct Item_ {
	pub super_:code_::Item1_,
	pub names_:code_::OL_,
	pub vals_:code_::OL_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self::new2(&kws.set_, &kws.equ_)
	}
	pub fn new2(kw:&keyword_::RI_, kw2:&keyword_::RI_) -> Self {
		Self {super_:code_::Item1_::new2(kw, kw2), names_:None, vals_:None}
	}
	
	pub fn hello2_1__(&self, names:T_<result_::List_>, vals:T_<result_::List_>, env:&code_::Env_) -> Result2_ {
		t_::o__(&self.names_).hello__(&code_::Env_::new6(names, env))?;
		if as_ref__!(env.w).dbg_.lc_ {
			use super::u_::code_::Item_;
			as_ref__!(env.w).dbg_.lc_kw__(t_::or__(&self.super_.kw2__()));
		}
		t_::o__(&self.vals_).hello__(&code_::Env_::new7(code_::Opt_ {vals_:true, ..env.gd}, vals, env))
	}
	pub fn hello2__(&self, is_alias:bool, env:&code_::Env_) -> Result2_ {
		let names = t__(result_::List_::new());
		let vals = t__(result_::List_::new());
		self.hello2_1__(names.clone(), vals.clone(), env)?;
		let names = as_ref__!(names);
		let vals = as_ref__!(vals);
		
		let mut name = String::new();
		let mut val = String::new();
		let mut val_rem = vec![];
		let mut val2 = result_::si__("");
		let mut yes_val2 = false;
		let mut i_name = 0;
		let mut i_val = 0;
		let mut cnt = 0;
		let mut is_priv = false;
		let mut douhao_in_end = false;
		loop {
			name.clear();
			let mut q2 = Some(env.q.clone());
			while i_name < names.len() {
				let i = as_ref__!(names[i_name]);
				i_name += 1;
				if i.dunhao__() {
					break
				}
				i.s_inc_some_kw__(&mut name);
				match self.super_.qv4rem__(&i.rem_, |rem| {
					if rem == "私" {
						is_priv = true;
						return true
					}
					false
				}, q2.unwrap(), env.w.clone()) {
					Ok(q3) => q2 = q3,
					Err(e) => return e,
				}
			}
			if i_val < vals.len() || douhao_in_end {
				val.clear();
				val_rem.clear();
				yes_val2 = false;
				if douhao_in_end {
					douhao_in_end = false
				} else {
					while i_val < vals.len() {
						let i2 = &vals[i_val];
						let i = as_ref__!(i2);
						i_val += 1;
						if i.dunhao__() {
							if i_val == vals.len() {
								douhao_in_end = true
							}
							break
						}
						if i.val_typ__() == "o" {
							val2 = i2.clone();
							yes_val2 = true;
							continue
						}
						i.s_inc_some_kw__(&mut val);
						for i2 in &i.rem_ {
							val_rem.push(i2.clone())
						}
					}
				}
			}
			qv_::val2__(&name, if yes_val2 {val2.clone()} else {result_::sri__(&val, val_rem.clone())},
				is_alias, is_priv, q2.unwrap().clone(), env.w.clone());
			cnt += 1;
			if env.gd.guandao_jie_ {
				if cnt > 1 {
					as_ref__!(env.w).dunhao__(&mut as_mut_ref__!(env.ret));
				}
				as_mut_ref__!(env.ret).add__(&name);
			}
			if i_name >= names.len() {
				break
			}
		}
		ok__()
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn kw2__(&self) -> keyword_::ORI_ {self.super_.kw2__()}

	fn add__(&mut self, a:code_::List_) -> Result2_ {
		self.names_ = Some(a);
		ok__()
	}
	fn add2__(&mut self, a:code_::List_) -> Result2_ {
		self.vals_ = Some(a);
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.names_)}
	fn a2__(&self) -> code_::ORL_ {t_::some__(&self.vals_)}

	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		self.hello2__(false, env)
	}
}
