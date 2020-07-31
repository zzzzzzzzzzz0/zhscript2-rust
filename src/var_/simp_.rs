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
	fn hello__(&self, env:&code_::Env_, _wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		if code_::attr_::get__(env.fa, &self.name_, false, ret) {
			return ok__()
		}
		match self.super_.super_.qv4rem__(&self.super_.rems_, |_| {false}, env.q.clone(), env.w.clone()) {
			Ok(q2) => {
				let q2 = q2.unwrap();
				let mut ret_alias = result_::List_::new();
				let mut q2 = q2;
				if qv_::get__(&self.name_, false, true, false, q2.clone(), env.w.clone(), ret, &mut ret_alias, &mut q2).is_some() {
					if !ret_alias.is_empty() {
						return self.super_.get__(&ret_alias, false, &code_::Env_::new2(q2, env), ret)
					}
					ok__()
				} else {
					self.super_.not_exist__(&self.name_, env.w.clone())
				}
			}
			Err(e) =>
				e,
		}
	}
}
