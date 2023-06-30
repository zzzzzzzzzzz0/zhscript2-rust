use super::{*, equ_name_::{equ_name__, EquName_}, super::{as_ref__, as_mut_ref__}};

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
	is_priv_:bool,
	gang_equ_name_:bool,
}

impl Item_ {
	pub fn name__(&self) -> &str {&self.name_}

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
		self.val2__(name, result_::si__(val), Typ_::Val, false, false)
	}
	pub fn val2__(&mut self, name:&str, val_:result_::I_, typ_:Typ_, is_priv_:bool, gang_equ_name_:bool) {
		if let Some(i) = self.a_.iter().find(|i| {as_ref__!(i).name__() == name}) {
			let i = &mut as_mut_ref__!(i);
			i.val_ = val_;
			i.typ2__(typ_);
			i.is_priv_ = is_priv_;
			i.gang_equ_name_ = gang_equ_name_;
		} else {
			self.a_.push(Rc_::new(RefCell_::new(Item_ {name_:name.to_string(), val_, typ_, is_priv_, gang_equ_name_})));
		}
	}
	
	pub fn get2__(&self, name:&str, is_has:bool, equ_name:&EquName_, no_self:bool, ret:&mut result_::List_, ret_alias:&mut result_::List_, ret_equ_name:&mut EquName_) -> bool {
		if let Some(i2) = self.a_.iter().find(|i| {
			let i = as_ref__!(i);
			if no_self && i.is_priv_ {
				return false
			}
			i.name__() == name
		}) {
			let i = as_ref__!(i2);
			if *i.typ__() == Typ_::Alias {
				if i.gang_equ_name_ {
					ret_equ_name.by__(name);
				}
				ret_alias.add4__(i.val_.clone());
				return true
			}
			if is_has {
				ret.add_r__("1", as_ref__!(i.val_).rem_.clone());
				return true
			}
			if i.gang_equ_name_ {
				let i3 = as_ref__!(i.val_);
				let val = &*as_ref__!(i3.val_);
				if let result_::Val_::S(s) = val {
					if equ_name__(s, name, ret) {
						ret_equ_name.by__(name);
						return true
					}
				}
			}
			if equ_name.equ_ {
				return true
			}
			ret.add4__(i.val_.clone());
			true
		} else {
			false
		}
	}
	#[allow(dead_code)]
	pub fn get__(&self, name:&str, is_has:bool, no_self:bool, ret:&mut result_::List_) -> bool {
		self.get2__(name, is_has, &Default::default(), no_self, ret, &mut result_::List_::new(), &mut Default::default())
	}
}
