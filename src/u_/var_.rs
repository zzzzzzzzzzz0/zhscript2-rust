use super::{*, super::{as_ref__, as_mut_ref__}};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Typ_ {
	Val,
	Alias,
}

pub type I_ = Rc_<RefCell_<Item_>>;

#[derive(Debug)]
pub struct Item_ {
	name_:String,
	val_:result_::I_,
	typ_:Typ_,
}

impl Item_ {
	pub fn name__(&self) -> &str {&self.name_}
	fn val__(&self) -> &result_::I_ {&self.val_}
	fn val2__(&mut self, s:result_::I_) {self.val_ = s}

	pub fn typ__(&self) -> &Typ_ {&self.typ_}
	fn typ2__(&mut self, typ:Typ_) {self.typ_ = typ}
}

#[derive(Clone)]
pub struct List_ {
	pub a_:Vec<I_>,
}

impl List_ {
	pub fn new() -> Self {Self {a_:vec![]}}
	
	pub fn val__(&mut self, name:&str, val:&str) {
		self.val2__(name, result_::si__(val), Typ_::Val)
	}
	pub fn val2__(&mut self, name:&str, val_:result_::I_, typ_:Typ_) {
		if let Some(i) = self.a_.iter().find(|i| {as_ref__!(i).name__() == name}) {
			let i = &mut as_mut_ref__!(i);
			i.val2__(val_);
			i.typ2__(typ_);
		} else {
			self.a_.push(Rc_::new(RefCell_::new(Item_ {name_:name.to_string(), val_, typ_})));
		}
	}
	
	pub fn get__(&self, name:&str, is_has:bool, ret:&mut result_::List_, ret_alias:&mut result_::List_) -> bool {
		if let Some(i2) = self.a_.iter().find(|i| {as_ref__!(i).name__() == name}) {
			let i = as_ref__!(i2);
			if *i.typ__() == Typ_::Alias {
				ret_alias.add4__(i.val__().clone())
			} else if is_has {
				ret.add_r__("1", as_ref__!(i.val__()).rem_.clone())
			} else {
				ret.add4__(i.val__().clone())
			}
			true
		} else {
			false
		}
	}
}
