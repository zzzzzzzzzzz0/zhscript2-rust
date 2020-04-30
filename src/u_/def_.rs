use super::{*, super::{/*lc3__, lc5__, lc_kw__, p__,*/ as_ref__, as_mut_ref__, cfg_if}};
use std::ops::Deref;

pub type ORL_<'a> = Option<&'a result_::List_>;
pub type OW_ = Option<world_::T_>;
pub type ORAN_ = Option<Rc_<ArgNames_>>;

#[derive(Debug)]
pub struct ArgName_ {
	pub s_:String,
	pub i_:usize,
	pub is_ge_:bool,
}

#[derive(Debug)]
pub struct ArgNames_ {
	a_:Vec<ArgName_>,
	pub has_ge_:bool,
}

impl Deref for ArgNames_ {
	type Target = Vec<ArgName_>;
	fn deref(&self) -> &Self::Target {&self.a_}
}

impl ArgNames_ {
	pub fn new(a:ORL_, w:OW_) -> Result<Self, Result2_> {
		let mut a_ = vec![];
		let mut has_ge_ = false;
		if let Some(a3) = a {
			let mut s = String::new();
			let mut is_ge = false;
			let mut cnt = 0;
			let mut empty = true;
			let w = w.unwrap();
			let w = as_ref__!(w);
			let mut add__ = |s:&mut String, cnt:&mut usize, is_ge:&mut bool, empty:&mut bool| {
				if !*empty {
					let i = if *is_ge {0} else {
						*cnt += 1;
						*cnt - 1
					};
					/*if lc_ {
						if *is_ge {
							lc5__!("\n{}", s);
						} else {
							lc3__!("\n{},{}", s, i);
						}
					}*/
					a_.push(ArgName_ {s_:s.to_string(), i_:i, is_ge_:*is_ge});
					s.clear();
					*is_ge = false;
					*empty = true;
				}
			};
			for i in a3.iter() {
				let i = as_ref__!(i);
				if i.dunhao__() {
					add__(&mut s, &mut cnt, &mut is_ge, &mut empty);
					continue
				}
				empty = false;
				i.s__(&mut s);
				for rem in &i.rem_ {
					if rem == "隔" {
						is_ge = true;
						has_ge_ = true;
						continue
					}
					return Err(w.no_rem2__(rem))
				}
			}
			add__(&mut s, &mut cnt, &mut is_ge, &mut empty);
		}
		Ok(Self {a_, has_ge_})
	}
}

//*mut u8
//*mut ::std::os::raw::c_void
pub type ObjTyp_ = *const std::ffi::c_void;
pub struct Obj_ {
	pub p_: ObjTyp_,
}
cfg_if! {
	if #[cfg(feature = "thread")] {
		unsafe impl Send for Obj_ {}
		unsafe impl Sync for Obj_ {}
	}
}
pub type BJS_ = Rc_<RefCell_<Vec<Obj_>>>;
pub type OBJS_ = Option<BJS_>;

#[allow(dead_code)]
//#[derive(Debug)]
pub enum Val_ {
	S(String),
	Si(String),
	F(fn(&code_::Opt_, qv_::T_, world_::T_, &mut WorldMut_, &mut result_::List_) -> Result2_),
}

pub type I_ = Rc_<RefCell_<Item_>>;

//#[derive(Debug)]
pub struct Item_ {
	name_:String,
	names_:ORAN_,
	pub val_:Val_,
	argc_:usize,
	arg0_:String,
	pub objs_:BJS_,
}

impl Item_ {
	pub fn new(name:&str, val_:Val_, argc_:usize, names_:ORAN_) -> Self {
		let mut arg0_ = name.to_string();
		if let Some(names) = &names_ {
			if names.has_ge_ {
				for i in names.iter() {
					arg0_ += if i.is_ge_ {
						&i.s_
					} else {
						"."
					};
				}
			}
		}
		Self {name_:name.to_string(), val_, names_, argc_, arg0_, objs_:Rc_::new(RefCell_::new(vec![]))}
	}

	pub fn name__(&self) -> &str {&self.name_}
	pub fn names__(&self) -> ORAN_ {
		self.names_.clone()
	}
	fn names2__(&mut self, a:ORL_, w:OW_) -> Result2_ {
		match ArgNames_::new(a, w) {
			Ok(names) => {
				self.names_ = Some(Rc_::new(names));
				ok__()
			}
			Err(e) =>
				e,
		}
	}
	pub fn arg0__(&self) -> &str {&self.arg0_}
	pub fn argc__(&self) -> usize {self.argc_}
	fn argc2__(&mut self, i:usize) {self.argc_ = i}

	#[allow(dead_code)]	
	pub fn objs_add__<T>(&mut self, i:&T) {
		unsafe {
			as_mut_ref__!(self.objs_).push(Obj_ {p_:std::mem::transmute::<*const T, ObjTyp_>(i)});
		}
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
	
	pub fn val__(&mut self, name:&str, val:&str, argc:usize, names:def_::ORL_, w:def_::OW_) -> Result2_ {
		self.val2__(name, def_::Val_::S(val.to_string()), argc, names, w)
	}
	pub fn val2__(&mut self, name:&str, val:def_::Val_, argc:usize, names:def_::ORL_, w:def_::OW_) -> Result2_ {
		if let Some(i) = self.a_.iter().find(|i| {as_ref__!(i).name__() == name}) {
			let mut di = as_mut_ref__!(i);
			di.val_ = val;
			di.names2__(names, w)?;
			di.argc2__(argc);
		} else {
			let names = match ArgNames_::new(names, w) {
				Ok(names) => Some(Rc_::new(names)),
				Err(e) => {
					return e
				}
			};
			self.add__(Item_::new(name, val, argc, names));
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
					Val_::S(s) => &s,
					Val_::Si(s) => &s,
					Val_::F(_) => "",
				})
			}
			true
		} else {
			false
		}
	}
	
	pub fn find__(&self, cs:&[char], from:usize, paichu:&Vec<String>) -> Option<(usize, usize, I_)> {
		let mut len = 0;
		if let Some(i) = self.a_.iter().find(|i| {
			let i = as_ref__!(i);
			if paichu.iter().any(|i2| i2 == i.arg0__()) {
				return false
			}
			if let Some(idx2) = t_::with__(cs, i.name__(), from) {
				len = idx2;
				return true
			}
			false
		}) {
			return Some((from, len, i.clone()))
		}
		None
	}
}
