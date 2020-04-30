use super::u_::*;

pub struct Item_ {
	super_:code_::Item1_,
	a_:code_::OL_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:code_::Item1_::new(&kws.name_), a_:None}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn add__(&mut self, a:code_::List_) -> Result2_ {Item1_::add__(self, a)}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}
	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		Item1_::hello__(self, gd, q, w, wm, ret)
	}
}

impl Item1_ for Item_ {
	fn codes2__(&mut self, a:code_::OL_) {self.a_ = a}
	fn codes__(&self) -> &code_::OL_ {&self.a_}
	fn can__(&self, _name:&String, _q:qv_::T_, _w:world_::T_) -> bool {true}
}

pub trait Item1_ : code_::Item_ {
	fn codes2__(&mut self, a:code_::OL_);
	fn codes__(&self) -> &code_::OL_;
	fn can__(&self, name:&String, q:qv_::T_, w:world_::T_) -> bool;

	fn add__(&mut self, a:code_::List_) -> Result2_ {
		self.codes2__(Some(a));
		ok__()
	}
	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, _ret:&mut result_::List_) -> Result2_ {
		let mut ret2 = result_::List_::new();
		t_::o__(self.codes__()).hello__(gd, q.clone(), w.clone(), wm, &mut ret2)?;
		let v = ret2.to_vec__();
		for name in v {
			if !self.can__(&name, q.clone(), w.clone()) {
				return result2_::err2__("无法命名")
			}
			let mut q = as_mut_ref__!(q);
			q.name_.push(name.to_string());
			q.src_ = name;
		}
		ok__()
	}
}
