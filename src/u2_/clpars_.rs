#![allow(dead_code)]

use super::str_;
//use regex::Regex;
use std::fmt::Write;

#[derive(PartialEq, Debug)]
pub enum Typ_ {
	Full,
	Starts,
	Ends,
	Has,
	Regex,
}

enum Tagv_ {
	S(Vec<String>),
	R(/*Regex*/bool),
}

type Cb1_ = dyn Fn(&Vec<String>, &Item_) -> ();
type Cb2_ = Box<Cb1_>;
type Cb_ = Option<Cb2_>;

pub struct Item_ {
	pub tag_:String,
	tagv_:Tagv_,
	pub argc_:usize,
	pub typ_:Typ_,
	cb_:Cb_,
	pub rem_:String,
}

const ARGC_Z_:usize = 1_000_000_000;

impl Item_ {
	//命名：tag、rem省，arg(c)、(t)yp、c(b)
	pub fn new0() -> Self {
		Self::new1c(0)
	}
	pub fn new0z() -> Self {
		Self::new1c(ARGC_Z_)
	}
	pub fn new(tag:&str) -> Self {
		Self::new2c(tag, 0)
	}
	pub fn new1(rem:&str) -> Self {
		Self::new2("", rem)
	}
	pub fn new1c(argc:usize) -> Self {
		Self::new2c("", argc)
	}
	pub fn new1b(cb:Cb_) -> Self {
		Self::new2b("", cb)
	}
	pub fn new2(tag:&str, rem:&str) -> Self {
		Self::new3(tag, 0, rem)
	}
	pub fn new2c(tag:&str, argc:usize) -> Self {
		Self::new3ct(tag, argc, Typ_::Full)
	}
	pub fn new2b(tag:&str, cb:Cb_) -> Self {
		Self::new3cb(tag, 0, cb)
	}
	pub fn new2t(tag:&str, typ:Typ_) -> Self {
		Self::new3ct(tag, 0, typ)
	}
	pub fn new2z(tag:&str, rem:&str) -> Self {
		Self::new3(tag, ARGC_Z_, rem)
	}
	pub fn new2zz(tag:&str, add:usize, rem:&str) -> Self {
		Self::new3(tag, ARGC_Z_ + add, rem)
	}
	pub fn new3(tag:&str, argc:usize, rem:&str) -> Self {
		Self::new__(tag, argc, Typ_::Full, None, rem)
	}
	pub fn new3t(tag:&str, typ:Typ_, rem:&str) -> Self {
		Self::new__(tag, 0, typ, None, rem)
	}
	pub fn new3cb(tag:&str, argc:usize, cb:Cb_) -> Self {
		Self::new4ctb(tag, argc, Typ_::Full, cb)
	}
	pub fn new3ct(tag:&str, argc:usize, typ:Typ_) -> Self {
		Self::new__(tag, argc, typ, None, "")
	}
	pub fn new3tb(tag:&str, typ:Typ_, cb:Cb_) -> Self {
		Self::new4ctb(tag, 0, typ, cb)
	}
	pub fn new4ctb(tag:&str, argc:usize, typ:Typ_, cb:Cb_) -> Self {
		Self::new__(tag, argc, typ, cb, "")
	}
	pub fn new__(tag:&str, argc:usize, typ:Typ_, cb:Cb_, rem:&str) -> Self {
		Self {tag_:tag.to_string(),
			tagv_:match typ {
				Typ_::Regex => Tagv_::R(/*Regex::new(&tag).unwrap()*/false),
				_ => Tagv_::S(str_::split2__(tag, "|", false, true, true)),
			},
			argc_:argc, typ_:typ, cb_:cb, rem_:rem.to_string()}
	}
	pub fn tag__(&self, tag:&str) -> i32 {
		if let Tagv_::S(v) = &self.tagv_ {
			let mut i2 = -1;
			for i in v.iter() {
				i2 += 1;
				if match self.typ_ {
					Typ_::Full => tag == i,
					Typ_::Starts => tag.starts_with(i),
					Typ_::Ends => tag.ends_with(i),
					Typ_::Has => tag.contains(i),
					_ => false
				} {
					return i2;
				}
			}
		}
		if let Tagv_::R(re) = &self.tagv_ {
			if *re/*.is_match(tag)*/ {
				return 0;
			}
		}
		-1
	}
}

pub struct List_ {
	pub a_:Vec<Item_>,
	pub rem_:String,
}

pub type A_ = dyn Iterator<Item = String>;

impl List_ {
	pub fn new(a:Vec<Item_>) -> Self {
		Self::new2(a, "")
	}
	pub fn new2(a:Vec<Item_>, rem:&str) -> Self {
		Self {a_:a, rem_:rem.to_string()}
	}
	
	pub fn add__(&self, i2:&Item_, a:&mut A_, argv:&mut Vec<String>) {
		for _ in 0..i2.argc_ {
			if let Some(i) = a.next() {
				argv.push(i.to_string());
			} else {
				break;
			}
		}
	}

	pub fn for__(&self, a:&mut A_,
			mut cb__:impl FnMut(&str, &Vec<String>, &Item_, u32) -> i32,
			mut cb2__:impl FnMut(&str) -> i32) -> i32 {
		let mut argv = vec![];
		while let Some(i) = a.next() {
			let mut i3 = 0;
			let mut cbx__ = |i2, i3:&mut u32| -> i32 {
				*i3 += 1;
				if *i3 == 1 {
					argv.clear();
					self.add__(i2, a, &mut argv);
					loop {
						let mut argc = i2.argc_;
						if i2.argc_ >= ARGC_Z_ {
							argc = i2.argc_ - ARGC_Z_;
							if argc == 0 {
								break;
							}
						}
						if argv.len() < argc {
							return 250;
						}
						if true {
							break;
						}
					}
				}
				cb__(&i, &argv, i2, *i3)
			};
			for i2 in self.a_.iter() {
				let i4 = i2.tag__(&i);
				if i4 >= 0 {
					let i5 = cbx__(i2, &mut i3);
					if i5 != 0 {
						return i5;
					}
				}
			}
			if i3 == 0 {
				for i2 in self.a_.iter() {
					if i2.tag_ == "" {
						let i5 = cbx__(i2, &mut i3);
						if i5 != 0 {
							return i5;
						}
					}
				}
				if i3 == 0 {
					let i5 = cb2__(&i);
					if i5 != 0 {
						return i5;
					}
				}
			}
		}
		0
	}

	pub fn for2__(&self, a:&mut A_) {
		let mut argv = vec![];
		while let Some(i) = a.next() {
			let mut i3 = 0;
			let cbx__ = |i2:&Item_, argv:&Vec<String>| {
				if let Some(cb__) = &i2.cb_ {
					cb__(&argv, i2);
				}
			};
			for i2 in self.a_.iter() {
				let i4 = i2.tag__(&i);
				if i4 >= 0 {
					i3 += 1;
					if i3 == 1 {
						argv.clear();
						self.add__(i2, a, &mut argv);
					}
					cbx__(&i2, &argv)
				}
			}
			if i3 == 0 {
				argv.clear();
				argv.push(i.to_string());
				for i2 in self.a_.iter() {
					if i2.tag_ == "" {
						cbx__(&i2, &argv)
					}
				}
			}
		}
	}
	
	pub fn help__(&self) -> String {
		let mut s = self.rem_.clone();
		for i in self.a_.iter() {
			write!(&mut s, "\n{}", i.tag_).unwrap();
			match i.argc_ {
				0 => {}
				_ => {
					write!(&mut s, " (").unwrap();
					if i.argc_ >= ARGC_Z_ {
						let argc = i.argc_ - ARGC_Z_;
						if argc > 0 {
							write!(&mut s, "{}", argc).unwrap();
						}
						write!(&mut s, "...").unwrap();
					} else {
						write!(&mut s, "{}", i.argc_).unwrap();
					}
					write!(&mut s, ")").unwrap();
				}
			}
			match i.typ_ {
				Typ_::Full => {}
				_ => write!(&mut s, "{}", format!(" ({:?})", i.typ_)).unwrap(),
			}
			write!(&mut s, " {}", i.rem_).unwrap();
		}
		s
	}
}
