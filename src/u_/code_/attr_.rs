#![allow(dead_code, unused_variables, unused_mut)]

use super::{*, };
use std::collections::HashMap;

pub type I_ = Rc_<Option<Item_>>;
type A_ = HashMap<String, String>;

pub fn i__(i:Option<Item_>) -> I_ {
	Rc_::new(i)
}

pub fn get__(i:I_, name:&str, is_has:bool, ret:&mut result_::List_) -> bool {
	let head = "属性";
	if name.starts_with(head) {
		let mut name = name[head.len()..].to_string();
		let mut i2 = i;
		while let Some(i) = &*i2 {
			let a = &i.a_;
			if let Ok(idx1) = name.parse::<usize>() {
				let idx = &i.idx_;
				if idx1 > 0 && idx1 <= idx.len() {
					name = idx[idx1 - 1].to_string();
				}
			} 
			if let Some(s) = a.get(&name) {
				if is_has {
					ret.add__("1");
				} else {
					ret.add__(s);
				}
				return true
			}
			i2 = i.up_.clone();
		}
	}
	false
}

pub struct Item_ {
	a_:A_,
	up_:I_,
	idx_:Vec<String>,
}

impl Item_ {
	pub fn new(up_:I_) -> Self {
		Self {up_, a_:A_::new(), idx_:vec![]}
	}
	pub fn add__<S:ToString>(&mut self, name:&str, val:S) {
		self.a_.insert(name.to_string(), val.to_string());
		let name = name.to_string();
		if !self.idx_.contains(&name) {
			self.idx_.push(name)
		}
	}
}
