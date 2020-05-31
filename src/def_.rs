use super::{u_::*, *};

pub struct Item_ {
	super_:set_::Item_,
	sp_:usize,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:set_::Item_::new2(&kws.def_, &kws.equ_), sp_:code_::Item1_::split2_0__()}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn kw2__(&self) -> keyword_::ORI_ {self.super_.kw2__()}

	fn add__(&mut self, a:code_::List_) -> Result2_ {
		self.super_.super_.chk_empty__(&a, "名缺")?;
		code_::Item1_::split2_1__(&a, &mut self.sp_);
		self.super_.add__(a)
	}
	fn add2__(&mut self, a:code_::List_) -> Result2_ {self.super_.add2__(a)}
	fn a__(&self) -> code_::ORL_ {self.super_.a__()}
	fn a2__(&self) -> code_::ORL_ {self.super_.a2__()}

	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, _ret:&mut result_::List_) -> Result2_ {
		let mut name = String::new();
		let mut argc = core::usize::MAX;
		let mut ret2 = result_::List_::new();
		let mut q2 = Some(q.clone());
		code_::Item1_::split2_2__(self.a__(), self.sp_, &mut name, |rem| {
			match rem {
				"无参" => argc = 0,
				_ => {
					match self.super_.super_.qv4rem_1__(rem, |_| {
						true
					}, q2.clone().unwrap(), w.clone()) {
						Ok(q3) => q2 = q3,
						Err(e) => return e,
					}
				}
			}
			ok__()
		}, code_::Opt_ {undef_eq_text_:true, ..gd}, q.clone(), w.clone(), wm, &mut ret2)?;
		
		if name.is_empty() {return self.super_.super_.err__("名空")}
		{
			let cs:Vec<char> = name.chars().collect();
			let paichu = vec![];
			if let Some(o) = qv_::for__(q.clone(), w.clone(), |q, _, _| {
				as_ref__!(q).defs_.find__(&cs, 0, false, false, &paichu)
			}) {
				let w = as_ref__!(w);
				return self.super_.super_.err__(&["名", &w.text__(&name), "被",
					&w.text__(as_ref__!(o.2).name__()), "覆盖"].concat())
			}
		}
		
		let mut ret3 = result_::List_::new();
		t_::o__(&self.a2__()).hello__(gd, q, w.clone(), wm, &mut ret3)?;
		if !ret2.is_empty() {
			argc = ret2.len__()
		}
		qv_::def__(&name, &ret3.s__(), argc, Some(&ret2), q2.unwrap(), w)
	}
}
