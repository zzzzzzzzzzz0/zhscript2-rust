use super::{u_::{*, equ_name_::EquName_}, as_ref__};

mod args_;
use args_::*;
mod argc_;
use argc_::*;
pub mod arg_;
use arg_::*;
pub mod simp_;

pub const WITH_NAME_:&str = "与名";

pub fn new(kws:&keyword_::List_, codes:&code_::List_) -> code_::OI_ {
	let mut bad = false;
	let mut name = String::new();
	let mut rem = String::new();
	let mut rems:Vec<String> = vec![];
	for i in codes.iter() {
		let (has, has2) = super::rem2_::text__(&i, &mut rem);
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
		if let Some((begin, end, line)) = Args_::with__(&name, &mut rems) {
			return code_::oi__(Args_::new(kws, rems, begin, end, line))
		}
		if Argc_::with__(&name) {
			return code_::oi__(Argc_::new(kws, rems))
		}
		if let Some(i) = Argi_::with__(&name) {
			if i == 0 {
				return code_::oi__(Arg0_::new(kws, rems, &name[ARG_.len()..]))
			} else {
				return code_::oi__(Argi_::new(kws, rems, i))
			}
		}
		return code_::oi__(simp_::Item_::new(kws, name, rems))
	}
	code_::oi__(Item_::new(kws, rems))
}

pub trait Name_ {
	fn name__(&self, _:&World_) -> String;
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

	fn s__(&self, mut s2:String, ret:&mut String, w:&World_) {
		for i in &self.rems_ {
			w.rem__(i, &mut s2);
		}
		self.super_.s2__(&s2, ret)
	}
	
	fn not_exist__(&self, name:&str, w:world_::T_) -> Result2_ {
		result2_::err__(["变量", &as_ref__!(w).text__(name), "不存在"].concat())
	}
	
	pub fn get2__(&self, ret2:&result_::List_, is_has:bool, equ_name:&EquName_, env:&code_::Env_) -> Result2_ {
		let mut name = String::new();
		let mut rems:Vec<String> = vec![];
		let mut with_name = false;
		let mut q2 = Some(env.q.clone());
		for i in ret2.iter() {
			let i = as_ref__!(i);
			i.s__(&mut name);
			match qv_::rem4_::hello2__(&i.rem_, is_has, |i2| {
				match i2 {
					WITH_NAME_ => with_name = true,
					_ => rems.push(i2.to_string())
				}
				true
			}, env.in_q_.clone(), q2.unwrap(), env.w.clone()) {
				Ok(q3) => {
					if q3.is_none() {return ok__()}
					q2 = q3
				}
				Err(e) => return e,
			}
		}
		if with_name {
			let mut s = name.clone();
			as_ref__!(env.w).rem1__(&mut s);
			as_mut_ref__!(env.ret).add__(s);
		}
		if !rems.is_empty() {
			if Args_::with2__(&name, &rems) {}
			else {
				return if is_has {ok__()} else {as_ref__!(env.w).no_rem2__(&rems[rems.len() - 1])}
			}
		}
		if code_::attr_::get__(env.fa.clone(), &name, is_has, &mut as_mut_ref__!(env.ret)) {
			return ok__()
		}
		let q2 = q2.unwrap();

		let get__ = |name, can_up| {
			let mut ret_alias = result_::List_::new();
			let mut equ_name2 = Default::default();
			let env2 = code_::Env_::new2(q2.clone(), env);
			let mut q2 = q2.clone();
			if qv_::get__(name, is_has, equ_name, can_up, can_up, &env2, &mut ret_alias, &mut equ_name2, &mut q2) {
				if !ret_alias.is_empty() {
					return Some(self.get2__(&ret_alias, is_has, &equ_name2, &code_::Env_::new2(q2, env)))
				}
				return Some(ok__())
			}
			None
		};

		if let Some(ret3) = get__(&name, false) {
			return ret3
		}

		if name.is_empty() {
			if let Some(ret3) = get__(&name, true) {
				return ret3
			}
		} else {
			if let Some((begin, end, line)) = Args_::with__(&name, &mut rems) {
				return Args_::hello__(is_has, q2, begin, end, line, &mut as_mut_ref__!(env.ret))
			}
			if Argc_::with__(&name) {
				return Argc_::hello__(is_has, q2, &mut as_mut_ref__!(env.ret))
			}
			if let Some(i) = Argi_::with__(&name) {
				if i == 0 {
					return Arg0_::hello__(&name[ARG_.len()..], is_has, q2, &mut as_mut_ref__!(env.ret))
				} else {
					Argi_::hello__(i, is_has, equ_name.equ_, &equ_name.name_, q2, &mut as_mut_ref__!(env.ret));
					return ok__()
				}
			}
			if let Some(ret3) = get__(&name, true) {
				return ret3
			}
		}
		
		if is_has {
			ok__()
		} else {
			self.not_exist__(&name, env.w.clone())
		}
	}
	pub fn get__(&self, ret2:&result_::List_, is_has:bool, env:&code_::Env_) -> Result2_ {
		self.get2__(ret2, is_has, &Default::default(), env)
	}

	pub fn hello__(&self, mut ok:impl FnMut(Option<qv_::T_>) -> Result2_, i:&dyn Name_, env:&code_::Env_) -> Result2_ {
		let mut with_name = false;
		match qv_::rem4_::hello__(&self.rems_, |rem| {
			match rem {
				WITH_NAME_ => {
					with_name = true;
					true
				}
				_ => false
			}
		}, env.in_q_.clone(), env.q.clone(), env.w.clone()) {
			Ok(q2) => {
				if with_name {
					let w = as_ref__!(env.w);
					let mut s = i.name__(&w);
					w.rem1__(&mut s);
					as_mut_ref__!(env.ret).add__(s);
				}
				ok(q2)
			}
			Err(e) => e,
		}
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

	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		let ret2 = t__(result_::List_::new());
		t_::o__(&self.a_).hello__(&code_::Env_::new6(ret2.clone(), env))?;
		#[cfg(debug_assertions)]
		if as_ref__!(env.w).dbg_.lc_ {
			as_ref__!(env.w).dbg_.lc_kw__(t_::or__(&self.super_.super_.kw2__()));
		}
		let ret2 = as_ref__!(ret2);
		self.super_.get__(&ret2, false, env)
	}
}
