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
	
	pub fn hello2_1__(&self, names:&mut result_::List_, vals:&mut result_::List_, gd:code_::Opt_,
			q:qv_::T_, w:world_::T_, wm:&mut WorldMut_) -> Result2_ {
		t_::o__(&self.names_).hello__(gd, q.clone(), w.clone(), wm, names)?;
		if wm.dbg_.lc_ {
			use super::u_::code_::Item_;
			wm.dbg_.lc_kw__(t_::or__(&self.super_.kw2__()));
		}
		t_::o__(&self.vals_).hello__(code_::Opt_ {vals_:true, ..gd}, q, w, wm, vals)
	}
	pub fn hello2__(&self, is_alias:bool, gd:code_::Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		let mut names = result_::List_::new();
		let mut vals = result_::List_::new();
		self.hello2_1__(&mut names, &mut vals, gd, q.clone(), w.clone(), wm)?;
		let mut name = String::new();
		let mut val = String::new();
		let mut val_rem = vec![];
		let mut val2 = result_::si__("");
		let mut yes_val2 = false;
		let mut i_name = 0;
		let mut i_val = 0;
		let mut cnt = 0;
		let mut is_priv = false;
		loop {
			name.clear();
			let mut q2 = Some(q.clone());
			while i_name < names.len() {
				let i = as_ref__!(names[i_name]);
				i_name += 1;
				if i.dunhao__() {
					break
				}
				i.s2__(&mut name, false, false, true);
				match self.super_.qv4rem__(&i.rem_, |rem| {
					if rem == "私" {
						is_priv = true;
						return true
					}
					false
				}, q2.unwrap(), w.clone()) {
					Ok(q3) => q2 = q3,
					Err(e) => return e,
				}
			}
			if i_val < vals.len() {
				val.clear();
				val_rem.clear();
				yes_val2 = false;
				while i_val < vals.len() {
					let i2 = &vals[i_val];
					let i = as_ref__!(i2);
					i_val += 1;
					if i.dunhao__() {
						break
					}
					if i.val_typ__() == "o" {
						val2 = i2.clone();
						yes_val2 = true;
						continue
					}
					i.s2__(&mut val, false, false, true);
					for i2 in &i.rem_ {
						val_rem.push(i2.clone())
					}
				}
			}
			qv_::val2__(&name, if yes_val2 {val2.clone()} else {result_::sri__(&val, val_rem.clone())},
				is_alias, is_priv, q2.unwrap().clone(), w.clone());
			cnt += 1;
			if gd.guandao_jie_ {
				if cnt > 1 {
					as_ref__!(w).dunhao__(ret);
				}
				ret.add__(&name);
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

	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		self.hello2__(false, gd, q, w, wm, ret)
	}
}
