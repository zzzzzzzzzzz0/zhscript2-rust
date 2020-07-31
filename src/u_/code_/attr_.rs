#![allow(dead_code)]

use super::{*, };
use std::collections::HashMap;

pub type ORI_ = Option<Rc_<Item_>>;
type A_ = HashMap<String, String>;

pub fn ori__(i:Item_) -> ORI_ {
	Some(Rc_::new(i))
}

pub fn get__(i:&ORI_, name:&str, is_has:bool, ret:&mut result_::List_) -> bool {
	let head = "属性";
	if name.starts_with(head) {
		let mut name = name[head.len()..].to_string();
		let mut i2 = i;
		while let Some(i) = i2 {
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
			i2 = &i.up_;
		}
	}
	false
}

pub struct Item_ {
	a_:A_,
	up_:ORI_,
	idx_:Vec<String>,
}

impl Item_ {
	pub fn new(up_:ORI_) -> Self {
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
