use super::{*, super::{as_ref__, as_mut_ref__, cfg_if}};
use std::{fmt, any::Any, ops::{Deref, DerefMut}};

pub fn si__<S:ToString>(s:S) -> I_ {
	sri__(s, vec![])
}
pub fn sri__<S:ToString>(s:S, rem_:Vec<String>) -> I_ {
	Rc_::new(RefCell_::new(Item_ {val_:sv__(s), rem_}))
}
pub fn v__(v:Val_) -> V_ {
	Rc_::new(RefCell_::new(v))
}
pub fn sv__<S:ToString>(s:S) -> V_ {
	v__(Val_::S(s.to_string()))
}

pub type I_ = Rc_<RefCell_<Item_>>;
pub type V_ = Rc_<RefCell_<Val_>>;
cfg_if! {
	if #[cfg(feature = "thread")] {
		type A_ = dyn Any + Send + Sync;
	} else {
		type A_ = dyn Any;
	}
}
type BA_ = Box<A_>;

#[derive(Debug)]
pub enum Val_ {
	S(String),
	K(keyword_::RI_),
	O(RefCell_<BA_>),
}

#[derive(Debug)]
pub struct Item_ {
	pub val_:V_,
	pub rem_:Vec<String>,
}

impl Item_ {
	pub fn val_typ__(&self) -> &str {
		let val = &*as_ref__!(self.val_);
		match val {
			Val_::S(_) => "s",
			Val_::K(_) => "k",
			Val_::O(_) => "o",
		}
	}
	
	pub fn s__(&self, s:&mut String) {
		self.s2__(s, false, false, false)
	}
	pub fn s2__(&self, s:&mut String, yin:bool, ma:bool, kw2:bool) {
		let val = &*as_ref__!(self.val_);
		if let Val_::S(s2) = val {
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
		if let Val_::K(kw) = val {
			if kw2 {
				match kw.id_ {
					keyword_::Id_::Jvhao => {}
					_ => s.push_str(&kw.s_)
				}
			}
			return
		}
	}
	
	pub fn dunhao__(&self) -> bool {
		if let Val_::K(kw) = &*as_ref__!(self.val_) {
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

#[derive(Clone, Default, Debug)]
pub struct List_ {
	pub a_:Vec<I_>,
	pub dunhao_:usize,
}

impl Deref for List_ {
	type Target = Vec<I_>;
	fn deref(&self) -> &Self::Target {&self.a_}
}
impl DerefMut for List_ {
	fn deref_mut(&mut self) -> &mut Vec<I_> {&mut self.a_}
}

impl List_ {
	pub fn new() -> Self {Default::default()}
	pub fn add__<S:ToString>(&mut self, s:S) {
		self.add_r__(s, vec![])
	}
	pub fn add_obj__(&mut self, i:BA_) {
		self.add_r2__(v__(Val_::O(RefCell_::new(i))), vec![])
	}
	/*pub fn add_obj2__(&mut self, i:A_) {
		self.add_r2__(v__(Val_::O(RefCell_::new(Box::new(i)))), vec![])
	}*/
	pub fn add_r__<S:ToString>(&mut self, s:S, rem_:Vec<String>) {
		self.add_r2__(sv__(s), rem_);
	}
	pub fn add_r2__(&mut self, val_:V_, rem_:Vec<String>) {
		self.add3__(Item_ {val_, rem_});
	}
	pub fn add2__(&mut self, kw:keyword_::RI_) {
		self.add3__(Item_ {val_:v__(Val_::K(kw)), rem_:vec![]});
	}
	pub fn add3__(&mut self, i:Item_) {
		self.add4__(Rc_::new(RefCell_::new(i)));
	}
	pub fn add4__(&mut self, i:I_) {
		if as_ref__!(i).dunhao__() {
			self.dunhao_ += 1
		}
		self.a_.push(i);
	}
	
	#[allow(dead_code)]
	pub fn obj__<T: 'static>(&self, i:usize, mut f:impl FnMut(&T)) -> bool {
		if let Val_::O(o) = &*as_ref__!(as_ref__!(self.a_[i]).val_) {
			if let Some(o) = as_ref__!(o).downcast_ref::<T>() {
				f(o);
				return true
			}
		}
		false
	}
	pub fn obj_mut__<T: 'static>(&self, i:usize, mut f:impl FnMut(&mut T)) -> bool {
		if let Val_::O(o) = &*as_ref__!(as_ref__!(self.a_[i]).val_) {
			if let Some(o) = as_mut_ref__!(o).downcast_mut::<T>() {
				f(o);
				return true
			}
		}
		false
	}
	
	pub fn to_vec__(&self) -> Vec<String> {
		self.to_vec2__(true)
	}
	pub fn to_vec2__(&self, otherkw:bool) -> Vec<String> {
		self.to_vec4__(self.a_.len(), otherkw)
	}
	pub fn to_vec3__(&self, len:usize) -> Vec<String> {
		self.to_vec4__(len, true)
	}
	pub fn to_vec4__(&self, len:usize, otherkw:bool) -> Vec<String> {
		let mut v = vec![];
		let mut s = String::new();
		let mut b = false;
		for i in 0..len {
			let i = &self.a_[i];
			let i = as_ref__!(i);
			let val = &*as_ref__!(i.val_);
			if let Val_::S(s2) = val {
				s.push_str(&s2);
				b = true;
				continue
			}
			if let Val_::K(kw) = val {
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
						}
						continue
					}
				}
			}
			if let Val_::O(_) = val {
				s.push_str("o");
				continue
			}
		}
		if b || !s.is_empty() {
			v.push(s);
		}
		v
	}
	
	pub fn to1__(&mut self, s1:&mut String) {
		if !self.a_.is_empty() {
			s1.push('"')
		}
		for i in &self.a_ {
			if let Val_::K(kw) = &*as_ref__!(as_ref__!(i).val_) {
				if kw.id_ == keyword_::Id_::Dunhao {
					s1.push_str("\" \"");
				}
				continue
			}
			as_ref__!(i).s2__(s1, false, true, false);
		}
		if !self.a_.is_empty() {
			s1.push('"')
		}
	}
	
	pub fn s__(&self) -> String {
		let mut s = String::new();
		for i in &self.a_ {
			as_ref__!(i).s__(&mut s)
		}
		s
	}
	
	pub fn end__(&self) -> Option<I_> {
		if self.a_.is_empty() {
			None
		} else {
			Some(self.a_[self.a_.len() - 1].clone())
		}
	}
}
