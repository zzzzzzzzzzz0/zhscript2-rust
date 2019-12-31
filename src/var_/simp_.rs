use super::*;

pub struct Item_ {
	super_:Item1_,
	name_:String,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_, name:String, rems:Vec<String>) -> Self {
		Self {super_:Item1_::new(&kws, rems), name_:name}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.super_.kw__()}
	fn s__(&self, ret:&mut String, w:&World_) {self.super_.s__(&self.name_, ret, w)}
	fn hello__(&self, _gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		match self.super_.super_.qv4rem__(&self.super_.rems_, |_| {false}, q, w) {
			Ok(q2) => {
				let q2 = q2.unwrap();
				if self.super_.argnames__(&self.name_, q2.clone(), ret) {
					return ok__()
				}
				qv_::get__(&self.name_, false, q2, w, ret)
			}
			Err(e) =>
				e,
		}
	}
}
