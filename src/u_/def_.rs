use super::{*, super::{as_ref__, as_mut_ref__}};

pub mod argname_;
use argname_::*;
pub mod backarg_;

pub type ORL_<'a> = Option<&'a result_::List_>;
pub type OW_ = Option<world_::T_>;
pub type ORAN_ = Option<Rc_<ArgNames_>>;

#[allow(dead_code)]
pub enum Val_ {
	S(String),
	Si(String),
	F(fn(&code_::Env_) -> Result2_),
}

pub type I_ = Rc_<RefCell_<Item_>>;

pub struct Item_ {
	name_:String,
	name0_:String,
	names_:ORAN_,
	pub val_:Val_,
	argc_:usize,
	pub backarg_:usize,
	pub objs_:qv_::BJS_,
}

impl Item_ {
	#[allow(dead_code)]
	pub fn new(name:&str, val:Val_, argc:usize, backarg:usize, names:ORAN_) -> Self {
		Self::new2(Self::mk_arg0__(name, &names), name.to_string(), val, argc, backarg, names)
	}
	fn new2(name_:String, name0_:String, val_:Val_, argc_:usize, backarg_:usize, names_:ORAN_) -> Self {
		Self {name_, val_, names_, argc_, backarg_, name0_, objs_:Rc_::new(RefCell_::new(vec![]))}
	}
	fn mk_arg0__(name:&str, names:&ORAN_) -> String {
		if let Some(names) = &names {
			if let Some(arg0) = Self::ch_arg0__(name, &names) {arg0} else {name.to_string()}
		} else {name.to_string()}
	}
	fn ch_arg0__(name:&str, names:&ArgNames_) -> Option<String> {
		if names.has_ge_ {
			let mut arg0 = name.to_string();
			for i in names.iter() {
				arg0 += if i.is_ge_ {
					&i.s_
				} else {
					"."
				};
			}
			return Some(arg0);
		}
		None
	}

	pub fn name__(&self) -> &str {&self.name_}
	pub fn names__(&self) -> ORAN_ {
		self.names_.clone()
	}
	pub fn argc__(&self) -> usize {self.argc_}

	#[allow(dead_code)]	
	pub fn objs_add__<T>(&mut self, i:&T) {
		qv_::objs_add__(&self.objs_, i)
	}
}

#[derive(Clone)]
pub struct List_ {
	pub a_:Vec<I_>,
}

impl List_ {
	pub fn new() -> Self {Self {a_:vec![]}}
	pub fn add__(&mut self, i:Item_) {
		self.a_.push(Rc_::new(RefCell_::new(i)));
	}
	
	pub fn val__(&mut self, name:&str, val:&str, argc:usize, backarg:usize, names:def_::ORL_, w:def_::OW_) -> Result2_ {
		self.val2__(name, def_::Val_::S(val.to_string()), argc, backarg, names, w)
	}
	pub fn val2__(&mut self, name:&str, val:def_::Val_, argc:usize, backarg:usize, names:def_::ORL_, w:def_::OW_) -> Result2_ {
		let names = match ArgNames_::new(names, w) {
			Ok(names) => Some(Rc_::new(names)),
			Err(e) => {
				return e
			}
		};
		let name1 = Item_::mk_arg0__(name, &names);
		if let Some(i) = self.a_.iter().find(|i| {as_ref__!(i).name_ == name1}) {
			let mut di = as_mut_ref__!(i);
			di.val_ = val;
			di.names_ = names;
			di.argc_ = argc;
			di.backarg_ = backarg;
		} else {
			self.add__(Item_::new2(name1, name.to_string(), val, argc, backarg, names));
		}
		ok__()
	}
	
	pub fn get__(&self, name:&str, is_has:bool, ret:&mut result_::List_) -> bool {
		if let Some(i) = self.a_.iter().find(|i| {as_ref__!(i).name__() == name}) {
			if is_has {
				ret.add__("1")
			} else {
				let i = as_ref__!(i);
				ret.add__(match &i.val_ {
					Val_::S(s) => s,
					Val_::Si(s) => s,
					Val_::F(_) => "",
				})
			}
			true
		} else {
			false
		}
	}
	
	pub fn find__(&self, cs:&[char], from:usize, can_eq:bool, _:bool, paichu:&[String]) -> Option<(usize, usize, I_)> {
		let mut len = 0;
		if let Some(i) = self.a_.iter().find(|i| {
			let i = as_ref__!(i);
			if paichu.iter().any(|i2| *i2 == i.name_) {
				return false
			}
			if let Some((idx2, eq)) = t_::with__(cs, &i.name0_, from) {
				if can_eq || !eq {
					len = idx2;
					return true
				}
			}
			false
		}) {
			return Some((from, len, i.clone()))
		}
		None
	}
}
