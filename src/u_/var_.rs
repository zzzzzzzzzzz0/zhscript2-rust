use super::*;
use std::rc::Rc;
use std::cell::RefCell;
//use core::any::Any;

	//fn as_any(&self) -> &dyn Any;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Typ_ {
	Val,
	Alias,
	Def,
}

pub type RRI_ = Rc<RefCell<dyn Item_>>;
pub trait Item_ {
	fn name__(&self) -> &str;
	fn val__(&self) -> &str;
	fn val2__(&mut self, s:&str);
	fn typ__(&self) -> &Typ_;

	//仅用于 def，怎么分离
	fn names__(&self) -> OArgNames_ {None}
	fn names2__(&mut self, _:def_::ORL_, _:def_::OW_) -> Result2_ {ok__()}
	fn arg0__(&self) -> &str {""}
	fn argc__(&self) -> usize {0}
	fn argc2__(&mut self, _i:usize) {}
	fn simp__(&self) -> bool {false}
	fn simp2__(&mut self, _b:bool) {}
}

#[derive(Debug)]
pub struct ItemV_ {
	name_:String,
	val_:String,
	typ_:Typ_,
}

impl Item_ for ItemV_ {
	fn name__(&self) -> &str {&self.name_}
	fn val__(&self) -> &str {&self.val_}
	fn val2__(&mut self, s:&str) {self.val_ = s.to_string()}

	fn typ__(&self) -> &Typ_ {&self.typ_}
}

pub struct List_ {
	a_:Vec<RRI_>,
}

impl List_ {
	pub fn new() -> Self {Self {a_:vec![]}}
	
	pub fn val__(&mut self, name:&str, val:&str) {
		self.val2__(name, val, Typ_::Val)
	}
	pub fn val2__(&mut self, name:&str, val:&str, typ:Typ_) {
		if let Some(i) = self.a_.iter().find(|i| {i.borrow().name__() == name}) {
			i.borrow_mut().val2__(val);
		} else {
			self.a_.push(Rc::new(RefCell::new(ItemV_ {name_:name.to_string(), val_:val.to_string(), typ_:typ})));
		}
	}
	
	pub fn def__(&mut self, name:&str, val:&str, argc:usize, names:def_::ORL_, w:def_::OW_) -> Result2_ {
		self.def2__(name, val, false, argc, names, w)
	}
	pub fn def2__(&mut self, name:&str, val:&str, simp:bool, argc:usize, names:def_::ORL_, w:def_::OW_) -> Result2_ {
		if let Some(i) = self.a_.iter().find(|i| {i.borrow().name__() == name}) {
			i.borrow_mut().val2__(val);
			i.borrow_mut().names2__(names, w)?;
			i.borrow_mut().argc2__(argc);
			i.borrow_mut().simp2__(simp);
		} else {
			let names = match ArgNames_::new(names, w) {
				Ok(names) => names,
				Err(e) => {
					return e
				}
			};
			self.a_.push(Rc::new(RefCell::new(def_::Item_::new(name, val, simp, argc, names))));
		}
		ok__()
	}
	
	pub fn get__(&self, name:&str, is_has:bool, ret:&mut result_::List_) -> bool {
		if let Some(i) = self.a_.iter().find(|i| {i.borrow().name__() == name}) {
			if is_has {
				ret.add__("1")
			} else {
				let i = i.borrow();
				if *i.typ__() == Typ_::Alias {
					return self.get__(i.val__(), false, ret)
				} else {
					ret.add__(i.val__())
				}
			}
			true
		} else {
			false
		}
	}
	
	pub fn find_def__(&self, cs:&[char], from:usize, end:usize, paichu:&Vec<String>) -> Option<(usize, usize, RRI_)> {
		let mut idx = from;
		let mut len = 0;
		//let end = if end > cs.len() {cs.len()} else {end};
		while idx < end {
			if let Some(i) = self.a_.iter().find(|i| {
				let i = i.borrow();
				if *i.typ__() == Typ_::Def {
					if paichu.iter().any(|i2| i2 == i.arg0__()) {
						return false
					}
					if let Some(idx2) = t_::with__(cs, i.name__(), idx) {
						len = idx2;
						return true
					}
				}
				false
			}) {
				return Some((idx, len, i.clone()))
			}
			idx += 1
		}
		None
	}
}
