use super::u_::*;

pub struct Item_ {
	super_:code_::Item1_,
	a_:code_::OL_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:code_::Item1_::new(&kws.mod_free_), a_:None}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn add__(&mut self, a:code_::List_) -> Result2_ {
		self.a_ = Some(a);
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}
	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		let mut ret2 = result_::List_::new();
		t_::o__(&self.a_).hello__(gd, q, w.clone(), wm, &mut ret2)?;
		let v = ret2.to_vec__();
		for name in v {
			let mut q2 = None;
			let mut i2 = None;
			{
				if let Some(i) = as_ref__!(w).mods_.iter().position(|q| {
					let is1 = as_ref__!(q).name_.contains(&name);
					if is1 {
						q2 = Some(q.clone());
					}
					is1
				}) {
					i2 = Some(i)
				}
			}
			if let Some(i) = i2 {
				let q2 = q2.unwrap();
				let on_free = &as_ref__!(q2).on_free_;
				if !on_free.is_empty() {
					eval_::hello__(on_free, gd, q2.clone(), w.clone(), wm, ret)?;
				}
				as_mut_ref__!(w).mods_.remove(i);
			}
		}
		ok__()
	}
}
