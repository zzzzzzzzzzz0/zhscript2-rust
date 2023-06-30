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
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		if code_::attr_::get__(env.fa.clone(), &self.name_, false, &mut as_mut_ref__!(env.ret)) {
			return ok__()
		}
		match code_::qv4rem__(&self.super_.rems_, |_| {false}, env.q.clone(), env.w.clone()) {
			Ok(q2) => {
				let q2 = q2.unwrap();
				let env2 = code_::Env_::new2(q2.clone(), env);
				let mut ret_alias = result_::List_::new();
				let mut equ_name2 = Default::default();
				let mut q2 = q2;
				if qv_::get__(&self.name_, false, &Default::default(), true, false, &env2, &mut ret_alias, &mut equ_name2, &mut q2) {
					if !ret_alias.is_empty() {
						return self.super_.get2__(&ret_alias, false, &equ_name2, &code_::Env_::new2(q2, env))
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
