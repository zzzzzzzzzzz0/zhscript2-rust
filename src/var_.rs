use super::{u_::*, rem2_, as_ref__};

mod args_;
use args_::*;
mod argc_;
use argc_::*;
mod arg_;
use arg_::*;
pub mod simp_;

pub fn new(kws:&keyword_::List_, codes:&code_::List_) -> code_::OI_ {
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
		match as_ref__!(i).kw__().id_ {
			keyword_::Id_::BeginText => {
				name.push_str(as_ref__!(i).s2__())
			}
			_ => {
				bad = true;
				break
			}
		}
	}
	if !bad {
		if let Some((begin, end)) = Args_::with__(&name, &mut rems) {
			return code_::oi__(Args_::new(kws, rems, begin, end))
		}
		if Argc_::with__(&name) {
			return code_::oi__(Argc_::new(kws, rems))
		}
		if Arg_::with__(&name) {
			return code_::oi__(Arg_::new(kws, rems))
		}
		if let Some(i) = Argi_::with__(&name) {
			if i == 0 {
				return code_::oi__(Arg0_::new(kws, rems))
			} else {
				return code_::oi__(Argi_::new(kws, rems, i))
			}
		}
		return code_::oi__(simp_::Item_::new(kws, name, rems))
	}
	code_::oi__(Item_::new(kws, rems))
}

pub struct Item1_ {
	pub super_:code_::Item1_,
	rems_:Vec<String>,
}
impl Item1_ {
	fn new(kws:&keyword_::List_, rems:Vec<String>) -> Self {
		Self::new2(code_::Item1_::new2(&kws.begin_var_, &kws.end_var_), rems)
	}
	pub fn new2(super_:code_::Item1_, rems_:Vec<String>) -> Self {
		Self {super_, rems_}
	}

	fn s__(&self, s:&str, ret:&mut String, w:&World_) {
		let mut s2 = String::new();
		s2.push_str(s);
		for i in &self.rems_ {
			w.rem__(i, &mut s2);
		}
		self.super_.s2__(&s2, ret)
	}
	
	fn not_exist__(&self, name:&str, w:world_::T_) -> Result2_ {
		result2_::err__(["变量", &as_ref__!(w).text__(name), "不存在"].concat())
	}
	
	
	pub fn get__(&self, ret2:&result_::List_, is_has:bool,
			env:&code_::Env_, ret:&mut result_::List_) -> Result2_ {
		let mut name = String::new();
		let mut rems:Vec<String> = vec![];
		let mut q2 = Some(env.q.clone());
		for i in ret2.iter() {
			let i = as_ref__!(i);
			i.s__(&mut name);
			match self.super_.qv4rem__(&i.rem_, |i2| {
				rems.push(i2.to_string());
				true
			}, q2.unwrap(), env.w.clone()) {
				Ok(q3) => q2 = q3,
				Err(e) => return e,
			}
		}
		if !rems.is_empty() {
			if Args_::with2__(&name, &rems) {}
			else {
				return if is_has {ok__()} else {result2_::err2__("注解不支持")}
			}
		}
		if code_::attr_::get__(env.fa, &name, is_has, ret) {
			return ok__()
		}
		let q2 = q2.unwrap();

		let get__ = |name, can_up, ret:&mut result_::List_| {
			let mut ret_alias = result_::List_::new();
			let mut q2 = q2.clone();
			if qv_::get__(name, is_has, can_up, can_up, q2.clone(), env.w.clone(), ret, &mut ret_alias, &mut q2).is_some() {
				if !ret_alias.is_empty() {
					return Some(self.get__(&ret_alias, is_has, &code_::Env_::new2(q2, env), ret))
				}
				return Some(ok__())
			}
			None
		};

		if let Some(ret3) = get__(&name, false, ret) {
			return ret3
		}

		if name.is_empty() {
			if let Some(ret3) = get__(&name, true, ret) {
				return ret3
			}
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
			if let Some(ret3) = get__(&name, true, ret) {
				return ret3
			}
		}
		
		self.not_exist__(&name, env.w.clone())
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
	pub fn new2(super_:Item1_) -> Self {
		Self {super_, a_:None}
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

	fn hello__(&self, env:&code_::Env_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		let mut ret2 = result_::List_::new();
		t_::o__(&self.a_).hello__(env, wm, &mut ret2)?;
		if wm.dbg_.lc_ {
			wm.dbg_.lc_kw__(t_::or__(&self.super_.super_.kw2__()));
		}
		self.super_.get__(&ret2, false, env, ret)
	}
}
