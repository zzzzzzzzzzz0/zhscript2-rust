use super::u_::*;
use super::rem2_;

mod args_;
use args_::*;
mod argc_;
use argc_::*;
mod arg_;
use arg_::*;
pub mod simp_;

pub fn new(kws:&keyword_::List_, codes:&code_::List_) -> Box<dyn code_::Item_ + 'static> {
	let mut bad = false;
	let mut name = String::new();
	let mut rem = String::new();
	let mut rems:Vec<String> = vec![];
	for i in codes.iter() {
		let (has, has2) = rem2_::text__(&i, &mut rem);
		if has {
			if has2 {
				rems.push(rem.to_string());
				continue
			} else {
				bad = true;
				break
			}
		}
		match i.kw__().id_ {
			keyword_::Id_::BeginText => {
				name.push_str(i.s2__())
			}
			_ => {
				bad = true;
				break
			}
		}
	}
	if !bad {
		if let Some((begin, end)) = Args_::with__(&name, &mut rems) {
			return Box::new(Args_::new(kws, rems, begin, end))
		}
		if Argc_::with__(&name) {
			return Box::new(Argc_::new(kws, rems))
		}
		if Arg_::with__(&name) {
			return Box::new(Arg_::new(kws, rems))
		}
		if let Some(i) = Argi_::with__(&name) {
			if i == 0 {
				return Box::new(Arg0_::new(kws, rems))
			} else {
				return Box::new(Argi_::new(kws, rems, i))
			}
		}
		return Box::new(simp_::Item_::new(kws, name, rems))
	}
	Box::new(Item_::new(kws, rems))
}

pub struct Item1_ {
	pub super_:code_::Item1_,
	rems_:Vec<String>,
}
impl Item1_ {
	fn new(kws:&keyword_::List_, rems:Vec<String>) -> Self {
		Self::new2(code_::Item1_::new2(&kws.begin_var_, &kws.end_var_), rems)
	}
	pub fn new2(i1:code_::Item1_, rems:Vec<String>) -> Self {
		Self {super_:i1, rems_:rems}
	}

	fn s__(&self, s:&str, ret:&mut String, w:&World_) {
		let mut s2 = String::new();
		s2.push_str(s);
		for i in &self.rems_ {
			w.rem__(i, &mut s2);
		}
		self.super_.s2__(&s2, ret)
	}
	
	fn argnames__(&self, name:&str, q2:qv_::T_, ret:&mut result_::List_) -> bool {
		if let Some(argnames) = &q2.borrow().argnames_ {
			for i in argnames.iter() {
				if i.is_ge_ {
					continue
				}
				if name == i.s_ {
					ret.add__(&q2.borrow().args2_[i.i_]);
					return true
				}
			}
		}
		false
	}
}

pub struct Item_ {
	pub super_:Item1_,
	a_:code_::OL_,
}

impl Item_ {
	fn new(kws:&keyword_::List_, rems:Vec<String>) -> Self {
		Self::new2(Item1_::new(&kws, rems))
	}
	pub fn new2(i1:Item1_) -> Self {
		Self {super_:i1, a_:None}
	}
	
	pub fn hello2__(&self, ret2:&result_::List_, is_has:bool,
			_:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		let mut name = String::new();
		let mut rems:Vec<String> = vec![];
		let mut q2 = Some(q);
		for i in ret2.iter() {
			let i = i.borrow();
			i.s__(&mut name);
			match self.super_.super_.qv4rem__(&i.rem_, |i2| {
				rems.push(i2.to_string());
				true
			}, q2.unwrap(), w) {
				Ok(q3) => q2 = q3,
				Err(e) => return e,
			}
		}
		let q2 = q2.unwrap();
		
		if self.super_.argnames__(&name, q2.clone(), ret) {
			return ok__()
		}
		
		if name.is_empty() {
			qv_::get__("", is_has, q2, w, ret)
		} else {
			if let Some((begin, end)) = Args_::with__(&name, &mut rems) {
				return Args_::hello__(is_has, q2, begin, end, ret)
			}
			if Argc_::with__(&name) {
				return Argc_::hello__(is_has, q2, ret)
			}
			if Arg_::with__(&name) {
				return Arg_::hello__(is_has, q2, ret)
			}
			if let Some(i) = Argi_::with__(&name) {
				if i == 0 {
					return Arg0_::hello__(is_has, q2, ret)
				} else {
					return Argi_::hello__(i, is_has, q2, ret)
				}
			}
			qv_::get__(&name, is_has, q2, w, ret)
		}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.super_.kw__()}
	fn kw2__(&self) -> keyword_::ORI_ {self.super_.super_.kw2__()}
	fn add__(&mut self, a:code_::List_) -> Result2_ {
		self.a_ = Some(a);
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}

	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		let mut ret2 = result_::List_::new();
		t_::o__(&self.a_).hello__(gd, q.clone(), w, &mut ret2)?;
		if w.dbg_.lc_ {
			w.dbg_.lc_kw__(t_::or__(&self.super_.super_.kw2__()));
		}
		self.hello2__(&ret2, false, gd, q, w, ret)
	}
}
