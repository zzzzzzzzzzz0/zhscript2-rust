use super::*;

pub struct Item_ {
	super_:Item1_,
	name_:String,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_, name_:String, rems:Vec<String>) -> Self {
		Self {super_:Item1_::new(&kws, rems), name_}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.super_.kw__()}
	fn s__(&self, ret:&mut String, w:&World_) {self.super_.s__(&self.name_, ret, w)}
	fn hello__(&self, _gd:code_::Opt_, q:qv_::T_, w:world_::T_, _wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		match self.super_.super_.qv4rem__(&self.super_.rems_, |_| {false}, q, w.clone()) {
			Ok(q2) => {
				let q2 = q2.unwrap();
				let mut ret_alias = result_::List_::new();
				let mut q2 = q2.clone();
				if qv_::get__(&self.name_, false, true, false, q2.clone(), w.clone(), ret, &mut ret_alias, &mut q2) {
					if !ret_alias.is_empty() {
						return self.super_.get__(&ret_alias, false, q2, w.clone(), ret)
					}
					ok__()
				} else {
					self.super_.not_exist__(&self.name_, w)
				}
			}
			Err(e) =>
				e,
		}
	}
}
