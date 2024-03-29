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
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		Item1_::hello__(self, env)
	}
}

impl Item1_ for Item_ {
	fn codes2__(&mut self, a:code_::OL_) {self.a_ = a}
	fn codes__(&self) -> &code_::OL_ {&self.a_}
	fn can__(&self, _:&String, argv:&Vec<String>, _:qv_::T_, _:world_::T_) -> bool {
		if argv.len() > 2 {
			return false
		}
		true
	}
}

pub trait Item1_ : code_::Item_ {
	fn codes2__(&mut self, a:code_::OL_);
	fn codes__(&self) -> &code_::OL_;
	fn can__(&self, name:&String, argv:&Vec<String>, q:qv_::T_, w:world_::T_) -> bool;

	fn add__(&mut self, a:code_::List_) -> Result2_ {
		self.codes2__(Some(a));
		ok__()
	}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		let ret2 = t__(result_::List_::new());
		t_::o__(self.codes__()).hello__(&code_::Env_::new6(ret2.clone(), env))?;
		let v = as_ref__!(ret2).to_vec__();
		let name = String::from(if v.is_empty() {""} else {&v[0]});
		if name.is_empty() || !self.can__(&name, &v, env.q.clone(), env.w.clone()) {
			return result2_::err__(["无法命名为", &as_ref__!(env.w).text__(&name)].concat())
		}
		let mut q = as_mut_ref__!(env.q);
		q.name_.push(name);
		if v.len() > 1 {
			q.on_free_.push_str(&v[1]);
		}
		ok__()
	}
}
