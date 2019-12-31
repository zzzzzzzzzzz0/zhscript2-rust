use super::u_::*;

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
	
	pub fn hello2__(&self, is_alias:bool, gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		let mut names = result_::List_::new();
		t_::o__(&self.names_).hello__(gd, q.clone(), w, &mut names)?;
		if w.dbg_.lc_ {
			use super::u_::code_::Item_;
			w.dbg_.lc_kw__(t_::or__(&self.super_.kw2__()));
		}
		let mut vals = result_::List_::new();
		t_::o__(&self.vals_).hello__(gd, q.clone(), w, &mut vals)?;
		let mut name = String::new();
		let mut val = String::new();
		let mut i_name = 0;
		let mut i_val = 0;
		let mut cnt = 0;
		loop {
			name.clear();
			let mut q2 = Some(q.clone());
			while i_name < names.len() {
				let i = names[i_name].borrow();
				i_name += 1;
				if i.dunhao__() {
					break
				}
				i.s__(&mut name);
				match self.super_.qv4rem__(&i.rem_, |_| {
					false
				}, q2.unwrap(), w) {
					Ok(q3) => q2 = q3,
					Err(e) => return e,
				}
			}
			if i_val < vals.len() {
				val.clear();
				while i_val < vals.len() {
					let i = vals[i_val].borrow();
					i_val += 1;
					if i.dunhao__() {
						break
					}
					i.s__(&mut val);
				}
			}
			qv_::val2__(&name, &val, is_alias, q2.unwrap().clone(), w);
			cnt += 1;
			if gd.guandao_jie_ {
				if cnt > 1 {
					w.dunhao__(ret);
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

	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		self.hello2__(false, gd, q, w, ret)
	}
}
