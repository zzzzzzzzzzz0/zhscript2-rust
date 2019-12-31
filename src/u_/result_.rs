use super::*;
use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;
use std::cell::RefCell;

pub type RRI_ = Rc<RefCell<Item_>>;

#[derive(Debug)]
pub enum Val_ {
	Str(String),
	Kw(keyword_::RI_),
}

#[derive(Debug)]
pub struct Item_ {
	pub val_:Val_,
	pub rem_:Vec<String>,
}

impl Item_ {
	pub fn s__(&self, s:&mut String) {
		self.s2__(s, false, false, false)
	}
	pub fn s2__(&self, s:&mut String, yin:bool, ma:bool, kw2:bool) {
		if let Val_::Str(s2) = &self.val_ {
			let mut s2 = s2.clone();
			if ma {
				s2 = s2.replace(r#"""#, r#"\""#)
			}
			if yin {
				s2 = format!(r#""{}""#, s2)
			}
			s.push_str(&s2);
			return
		}
		if let Val_::Kw(kw) = &self.val_ {
			if kw2 {
				s.push_str(&kw.s_);
			}
			return
		}
	}
	
	pub fn dunhao__(&self) -> bool {
		if let Val_::Kw(kw) = &self.val_ {
			if kw.id_ == keyword_::Id_::Dunhao {
				 return true
			}
		}
		false
	}
}

/*impl ToString for Item_ {
	fn to_string(&self) -> String {
		s
	}
}*/
impl fmt::Display for Item_ {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut s = String::new();
		self.s2__(&mut s, true, true, true);
		write!(f, "{}", s)
	}
}

#[derive(Debug)]
pub struct List_ {
	pub a_:Vec<RRI_>,
	pub dunhao_:usize,
}

impl Deref for List_ {
	type Target = Vec<RRI_>;
	fn deref(&self) -> &Self::Target {&self.a_}
}
impl DerefMut for List_ {
	fn deref_mut(&mut self) -> &mut Vec<RRI_> {&mut self.a_}
}

impl List_ {
	pub fn new() -> Self {
		Self {a_:vec![], dunhao_:0}
	}
	pub fn add__<S:ToString>(&mut self, s:S) {
		self.add3__(Item_ {val_:Val_::Str(s.to_string()), rem_:vec![]});
	}
	pub fn add2__(&mut self, kw:keyword_::RI_) {
		self.add3__(Item_ {val_:Val_::Kw(kw), rem_:vec![]});
	}
	pub fn add3__(&mut self, i:Item_) {
		self.add4__(Rc::new(RefCell::new(i)));
	}
	pub fn add4__(&mut self, i:RRI_) {
		if i.borrow().dunhao__() {
			self.dunhao_ += 1
		}
		self.a_.push(i);
	}
	
	pub fn to_vec__(&self) -> Vec<String> {
		self.to_vec2__(true)
	}
	pub fn to_vec2__(&self, otherkw:bool) -> Vec<String> {
		let mut v = vec![];
		let mut s = String::new();
		let mut b = false;
		for i in &self.a_ {
			let i = i.borrow();
			if let Val_::Str(s2) = &i.val_ {
				s.push_str(&s2);
				b = true;
				continue
			}
			if let Val_::Kw(kw) = &i.val_ {
				match kw.id_ {
					keyword_::Id_::Dunhao => {
						v.push(s.to_string());
						s.clear();
						b = true;
						continue
					}
					keyword_::Id_::Jvhao |
					keyword_::Id_::Douhao =>
						continue,
					_ => {
						if otherkw {
							s.push_str(&kw.s_);
							b = true;
						}
						continue
					}
				}
			}
		}
		if b {
			v.push(s.to_string());
		}
		v
	}
	
	pub fn to1__(&mut self, s1:&mut String) {
		if !self.a_.is_empty() {
			s1.push('"')
		}
		for i in &self.a_ {
			if let Val_::Kw(kw) = &i.borrow().val_ {
				if kw.id_ == keyword_::Id_::Dunhao {
					s1.push_str("\" \"");
				}
				continue
			}
			i.borrow().s2__(s1, false, true, false);
		}
		if !self.a_.is_empty() {
			s1.push('"')
		}
	}
	
	pub fn s__(&self) -> String {
		let mut s = String::new();
		for i in &self.a_ {
			i.borrow().s__(&mut s)
		}
		s
	}
	
	pub fn end__(&self) -> Option<RRI_> {
		if self.a_.is_empty() {
			None
		} else {
			Some(self.a_[self.a_.len() - 1].clone())
		}
	}
}
