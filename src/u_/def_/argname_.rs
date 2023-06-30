use super::{*, };
use std::ops::Deref;

pub const SP_:&str = "隔";

#[derive(Debug)]
pub struct ArgName_ {
	pub s_:String,
	pub i_:i32,
	pub is_ge_:bool,
	pub equ_name_:bool,
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
			let mut equ_name_ = false;
			let mut cnt = 0;
			let mut empty = true;
			let w = w.unwrap();
			let w = as_ref__!(w);
			let mut add__ = |s:&mut String, cnt:&mut i32, is_ge:&mut bool, equ_name_, empty:&mut bool| {
				if !*empty {
					let i_ = if *is_ge {0} else {
						*cnt += 1;
						*cnt
					};
					a_.push(ArgName_ {s_:s.to_string(), i_, is_ge_:*is_ge, equ_name_});
					s.clear();
					*is_ge = false;
					*empty = true;
				}
			};
			for i in a3.iter() {
				let i = as_ref__!(i);
				if i.dunhao__() {
					add__(&mut s, &mut cnt, &mut is_ge, equ_name_, &mut empty);
					continue
				}
				empty = false;
				i.s__(&mut s);
				for rem in &i.rem_ {
					match rem.as_str() {
						SP_ => {
							is_ge = true;
							has_ge_ = true;
						}
						equ_name_::REM_ => {
							equ_name_ = true;
						}
						_ => return Err(w.no_rem2__(rem))
					}
				}
			}
			add__(&mut s, &mut cnt, &mut is_ge, equ_name_, &mut empty);
		}
		Ok(Self {a_, has_ge_})
	}
}
