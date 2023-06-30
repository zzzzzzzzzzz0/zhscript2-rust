use super::{*, super::{as_ref__, as_mut_ref__, cfg_if}};
use std::{fmt, any::Any};

pub fn si__<S:ToString>(s:S) -> I_ {
	sri__(s, vec![])
}
pub fn sri__<S:ToString>(s:S, rem:Vec<String>) -> I_ {
	vri__(sv__(s), rem)
}
pub fn vri__(val_:V_, rem_:Vec<String>) -> I_ {
	Rc_::new(RefCell_::new(Item_ {val_, rem_}))
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
		type O_ = dyn Any + Send + Sync;
	} else {
		type O_ = dyn Any;
	}
}
pub type BO_ = Box<O_>;
pub type RBO_ = RefCell_<BO_>;
pub type RO_ = Rc_<RefCell_<BO_>>;
#[allow(dead_code)]
pub fn oi__(i:BO_) -> I_ {
	vri__(v__(ov__(i)), vec![])
}
fn ov__(i:BO_) -> Val_ {
	Val_::O(RefCell_::new(i))
}

#[derive(Debug)]
pub enum Val_ {
	S(String),
	K(keyword_::RI_),
	O(RBO_),
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
		self.s2__(s, false, false, false, false)
	}
	pub fn s2__(&self, s:&mut String, yin:bool, ma:bool, kw2:bool, kw3:bool) {
		self.s3__(s, yin, ma, kw2, kw3, false, false);
	}
	pub fn s3__(&self, s:&mut String, yin:bool, ma:bool, kw2:bool, kw3:bool, kw4:bool, kw5:bool) -> u8 {
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
			return 1
		}
		if let Val_::K(kw) = val {
			if kw2 {
				if !kw3 && kw.id_ == keyword_::Id_::Jvhao {
					return 0
				}
				if kw4 {
					match kw.id_ {
						keyword_::Id_::Jvhao |
						keyword_::Id_::Dunhao => return b',',
						keyword_::Id_::Douhao => return b'.',
						_ => if !kw5 {return 0}
					}
				}
				s.push_str(&kw.s_);
				return 1
			}
			return 0
		}
		if let Val_::O(_) = val {
			s.push_str("obj");
			return 1
		}
		0
	}
	pub fn s_inc_some_kw__(&self, s:&mut String) {
		self.s2__(s, false, false, true, false)
	}
	
	pub fn dunhao__(&self) -> bool {
		self.is_kw__(keyword_::Id_::Dunhao)
	}
	pub fn jvhao__(&self) -> bool {
		self.is_kw__(keyword_::Id_::Jvhao)
	}
	pub fn is_kw__(&self, id:keyword_::Id_) -> bool {
		if let Val_::K(kw) = &*as_ref__!(self.val_) {
			if kw.id_ == id {
				 return true
			}
		}
		false
	}
}

impl fmt::Display for Item_ {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut s = String::new();
		self.s2__(&mut s, true, true, true, false);
		write!(f, "{}", s)
	}
}

#[derive(Clone, Default, Debug)]
pub struct List_ {
	pub a_:Vec<I_>,
	pub dunhao_:Vec<usize>,
}

impl std::ops::Deref for List_ {
	type Target = Vec<I_>;
	fn deref(&self) -> &Self::Target {&self.a_}
}

impl List_ {
	pub fn new() -> Self {Default::default()}
	pub fn clear(&mut self) {
		self.a_.clear();
		self.dunhao_.clear();
	}
	pub fn add__<S:ToString>(&mut self, s:S) {
		self.add_r__(s, vec![])
	}
	pub fn add_obj__(&mut self, i:BO_) {
		self.add_r2__(v__(ov__(i)), vec![])
	}
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
			self.dunhao_.push(self.a_.len())
		}
		self.a_.push(i);
	}
	pub fn ins__(&mut self, i:I_) {
		let mut skip0 = if as_ref__!(i).dunhao__() {
			self.dunhao_.insert(0, 0);
			true
		} else {false};
		self.a_.insert(0, i);
		for i in &mut self.dunhao_ {
			if skip0 {
				skip0 = false;
				continue;
			}
			*i += 1;
		}
	}
	pub fn rm__(&mut self, idx:usize) -> I_ {
		/*self.dunhao_.retain_mut(|i| {
			if *i > idx {
				*i -= 1;
				true
			} else {
				*i < idx
			}
		});*/
		self.dunhao_.retain(|i| *i != idx);
		for i in &mut self.dunhao_ {
			if *i > idx {
				*i -= 1;
			}
		}
		self.a_.remove(idx)
	}

	pub fn len__(&self) -> usize {
		if self.a_.is_empty() {0} else {self.dunhao_.len() + 1}
	}
	
	#[allow(dead_code)]
	pub fn obj__<T: 'static>(&self, i:usize, mut f:impl FnMut(&T) -> bool) -> bool {
		if let Val_::O(o) = &*as_ref__!(as_ref__!(self.a_[i]).val_) {
			if let Some(o) = as_ref__!(o).downcast_ref::<T>() {
				return f(o)
			}
		}
		false
	}
	pub fn obj_mut__<T: 'static>(&self, i:usize, mut f:impl FnMut(&mut T) -> bool) -> bool {
		if let Val_::O(o) = &*as_ref__!(as_ref__!(self.a_[i]).val_) {
			if let Some(o) = as_mut_ref__!(o).downcast_mut::<T>() {
				return f(o)
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
		self.to_vec0__(0, len, otherkw)
	}
	#[allow(dead_code)]
	pub fn to_vec5__(&self, start:usize) -> Vec<String> {
		self.to_vec0__(start, self.a_.len(), true)
	}
	pub fn to_vec0__(&self, start:usize, len:usize, otherkw:bool) -> Vec<String> {
		let mut v = vec![];
		let mut s = String::new();
		let mut b = false;
		for i in start..len {
			let i = &self.a_[i];
			match as_ref__!(i).s3__(&mut s, false, false, true, true, true, otherkw) {
				1 => b = true,
				b',' => {
					v.push(s.to_string());
					s.clear();
					b = true;
				}
				_ => {}
			}
		}
		if b || !s.is_empty() {
			v.push(s);
		}
		v
	}
	
	pub fn s2s__(s2:&mut String, ctl:u8, s1:&mut String) {
		let mut yin = false;
		match ctl {
			b'0' => {}
			_ => {
				yin = s2.is_empty();
				if !yin {
					match ctl {
						b'j' =>
						match s2.as_str() {
							"true" | "false" | "null" | "undefined" => {}
							_ => yin = true
						}
						_ =>
						for c in s2.as_bytes() {
							match c {
								b'0'..=b'9' | b'.' |
								b'a'..=b'z' |
								b'A'..=b'Z' |
								b'-' | b'_' => {}
								_ => {
									yin = true;
									break
								}
							}
						}
					}
				}
			}
		}
		if !s1.is_empty() {
			s1.push_str(match ctl {
				b'j' => ",",
				b'0' => "|\u{c}",
				_ => " ",
			})
		}
		if yin {s1.push('"')}
		s1.push_str(&s2);
		if yin {s1.push('"')}
		s2.clear();
	}
	pub fn s2__(&self, ctl:u8, s1:&mut String) {
		if self.a_.is_empty() {return}
		let mut s2 = String::new();
		let to1__ = |s1:&mut String, s2:&mut String| {
			Self::s2s__(s2, ctl, s1);
		};
		for i in &self.a_ {
			if let Val_::K(kw) = &*as_ref__!(as_ref__!(i).val_) {
				if kw.id_ == keyword_::Id_::Dunhao || kw.id_ == keyword_::Id_::Jvhao {
					to1__(s1, &mut s2);
				}
				continue
			}
			as_ref__!(i).s2__(&mut s2, false, match ctl {
				b'0' => false,
				_ => true
			}, false, false);
		}
		to1__(s1, &mut s2);
		if ctl == b'j' && self.len__() > 1 {
			*s1 = format!("[{}]", s1);
		}
	}
	pub fn to1__(&self, s1:&mut String) {
		self.s2__(b'1', s1)
	}
	
	pub fn s__(&self) -> String {
		let mut s = String::new();
		for i in &self.a_ {
			as_ref__!(i).s__(&mut s)
		}
		s
	}
	
	pub fn s3__(&self, i:usize) -> Option<String> {
		if let Some((start, end)) = self.posi__(i) {
			let mut s = String::new();
			for idx in start..end {
				if let Val_::S(s2) = &*as_ref__!(as_ref__!(self.a_[idx]).val_) {
					s.push_str(s2)
				} else {
					return None
				}
			}
			return Some(s)
		}
		None
	}
	
	pub fn ret__(&self, i:usize, ret:&mut Self) {
		if let Some((start, end)) = self.posi__(i) {
			for idx in start..end {
				ret.add4__(self.a_[idx].clone())
			}
		}
	}
	
	fn posi__(&self, i:usize) -> Option<(usize, usize)> {
		let mut end2 = self.len__();
		if end2 > 0 {
			end2 -= 1;
			let start = if i > 0 && i <= end2 {
					self.dunhao_[i - 1] + 1
				} else {
					0
				};
			let end = if i >= end2 {self.a_.len()} else {self.dunhao_[i]};
			if start < end {
				return Some((start, end))
			}
		}
		None
	}
	
	pub fn end__(&self) -> Option<I_> {
		if self.a_.is_empty() {
			None
		} else {
			Some(self.a_[self.a_.len() - 1].clone())
		}
	}
	pub fn pop_end__(&mut self) {
		if let Some(i) = self.end__() {
			if as_ref__!(i).dunhao__() {
				self.a_.pop();
				self.dunhao_.pop();
			}
		}
	}
}
