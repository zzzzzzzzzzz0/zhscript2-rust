use super::*;
use super::super::*;
use std::rc::Rc;
use std::ops::Deref;

pub type ORL_<'a> = Option<&'a result_::List_>;
pub type OW_<'a> = Option<&'a World_>;
type RAN_ = Rc<ArgNames_>;
pub type ORAN_ = Option<RAN_>;

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
		let mut a2 = vec![];
		let mut has_ge = false;
		if let Some(a3) = a {
			let mut s = String::new();
			let mut is_ge = false;
			let mut cnt = 0;
			let mut empty = true;
			let w = w.unwrap();
			let mut add__ = |s:&mut String, cnt:&mut usize, is_ge:&mut bool, empty:&mut bool| {
				if !*empty {
					let i = if *is_ge {0} else {
						*cnt += 1;
						*cnt - 1
					};
					if w.dbg_.lc_ {
						if *is_ge {
							lc5__!("\n{}", s);
						} else {
							lc3__!("\n{},{}", s, i);
						}
					}
					a2.push(ArgName_ {s_:s.to_string(), i_:i, is_ge_:*is_ge});
					s.clear();
					*is_ge = false;
					*empty = true;
				}
			};
			for i in a3.iter() {
				let i = i.borrow();
				if i.dunhao__() {
					add__(&mut s, &mut cnt, &mut is_ge, &mut empty);
					continue
				}
				empty = false;
				i.s__(&mut s);
				for rem in &i.rem_ {
					if rem == "隔" {
						is_ge = true;
						has_ge = true;
						continue
					}
					return Err(w.no_rem2__(rem))
				}
			}
			add__(&mut s, &mut cnt, &mut is_ge, &mut empty);
		}
		Ok(Self {a_:a2, has_ge_:has_ge})
	}
}

#[derive(Debug)]
pub struct Item_ {
	name_:String,
	names_:RAN_,
	val_:String,
	argc_:usize,
	arg0_:String,
	simp_:bool,
}

impl Item_ {
	pub fn new(name:&str, val:&str, simp:bool, argc:usize, names:ArgNames_) -> Self {
		let mut arg0 = name.to_string();
		for i in names.iter() {
			arg0 += if i.is_ge_ {
				&i.s_
			} else {
				"."
			};
		}
		Self {name_:name.to_string(), val_:val.to_string(), names_:Rc::new(names),
				argc_:argc, arg0_:arg0, simp_:simp}
	}
}

impl super::var_::Item_ for Item_ {
	fn name__(&self) -> &str {&self.name_}
	fn names__(&self) -> ORAN_ {
		Some(self.names_.clone())
	}
	fn names2__(&mut self, a:ORL_, w:OW_) -> Result2_ {
		match ArgNames_::new(a, w) {
			Ok(names) => {
				self.names_ = Rc::new(names);
				ok__()
			}
			Err(e) =>
				e,
		}
	}
	fn arg0__(&self) -> &str {&self.arg0_}
	fn argc__(&self) -> usize {self.argc_}
	fn argc2__(&mut self, i:usize) {self.argc_ = i}
	fn val__(&self) -> &str {&self.val_}
	fn val2__(&mut self, s:&str) {self.val_ = s.to_string()}
	fn simp__(&self) -> bool {self.simp_}
	fn simp2__(&mut self, b:bool) {self.simp_ = b}

	fn typ__(&self) -> &super::var_::Typ_ {&super::var_::Typ_::Def}
}
