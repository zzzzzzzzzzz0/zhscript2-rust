#![allow(dead_code)]

use super::str_;
use std::fmt::{self, Write};

pub trait Typ2_ : fmt::Debug {
	fn s__(&self) -> &'static str;
	fn c__(&self) -> char;
	fn can_split__(&self) -> bool {true}
	fn with__(&self, tag:&str, i:&str, argv:&mut Vec<String>) -> bool;
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {f.pad(self.s__())}
}

#[derive(Debug)]
pub enum Typ_ {
	Full,
	Starts,
	Ends,
	Has,
	X(Box<dyn Typ2_>),
}

type Cb1_ = dyn Fn(&Vec<String>, &Item_) -> ();
type Cb2_ = Box<Cb1_>;
pub enum Cb_ {
	F(Cb2_),
	S(String),
	No
}

pub struct Item_ {
	pub tag_:String,
	pub tagv_:Vec<String>,
	pub argc_:usize,
	pub typ_:Typ_,
	pub cb_:Cb_,
	pub rem_:String,
}

unsafe impl Send for Item_ {}
unsafe impl Sync for Item_ {}

pub const ARGC_Z_:usize = 1_000_000_000;

pub const ARG_NE_:i32 = 250;
pub const   HELP_:i32 = 251;
pub const TAG_NO_:i32 = 252;

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
	pub fn new1z(tag:&str) -> Self {
		Self::new2z(tag, "")
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
	pub fn new2cz(tag:&str, add:usize) -> Self {
		Self::new3z(tag, add, "")
	}
	pub fn new3z(tag:&str, add:usize, rem:&str) -> Self {
		Self::new3(tag, ARGC_Z_ + add, rem)
	}
	pub fn new3(tag:&str, argc:usize, rem:&str) -> Self {
		Self::new__(tag, argc, Typ_::Full, Cb_::No, rem)
	}
	pub fn new3t(tag:&str, typ:Typ_, rem:&str) -> Self {
		Self::new__(tag, 0, typ, Cb_::No, rem)
	}
	pub fn new3cb(tag:&str, argc:usize, cb:Cb_) -> Self {
		Self::new4ctb(tag, argc, Typ_::Full, cb)
	}
	pub fn new3ct(tag:&str, argc:usize, typ:Typ_) -> Self {
		Self::new__(tag, argc, typ, Cb_::No, "")
	}
	pub fn new3tb(tag:&str, typ:Typ_, cb:Cb_) -> Self {
		Self::new4ctb(tag, 0, typ, cb)
	}
	pub fn new4ctb(tag:&str, argc:usize, typ:Typ_, cb:Cb_) -> Self {
		Self::new__(tag, argc, typ, cb, "")
	}
	pub fn new__(tag:&str, argc_:usize, typ_:Typ_, cb_:Cb_, rem:&str) -> Self {
		Self {tag_:tag.to_string(),
			tagv_:if match &typ_ {
				Typ_::X(x) => x.can_split__(),
				_ => true,
			} {str_::split2__(tag, "|", false, true, true)} else {vec![tag.to_string()]},
			argc_, typ_, cb_, rem_:rem.to_string()}
	}
	pub fn tag__(&self, tag:&str, argv:&mut Vec<String>) -> i32 {
		let mut i2 = -1;
		for i in self.tagv_.iter() {
			i2 += 1;
			if i.is_empty() {continue}
			if match &self.typ_ {
				Typ_::Full => tag == i,
				Typ_::Starts => tag.starts_with(i), //与下皆不能像c版那样传截，影响及他
				Typ_::Ends   => tag.  ends_with(i),
				Typ_::Has    => tag.   contains(i),
				Typ_::X(x) => x.with__(tag, i, argv),
			} {
				return i2;
			}
		}
		-1
	}
	pub fn tag1__(&self) -> &str {
		if self.tagv_.is_empty() {""} else {&self.tagv_[0]}
	}
}

pub struct List_ {
	pub a_:Vec<Item_>,
	pub rem_:String,
}

pub type A_ = dyn Iterator<Item = String>;
pub type Result_ = std::result::Result<(), (i32, String)>;

impl List_ {
	pub fn new() -> Self {
		Self::new2(vec![])
	}
	pub fn new2(a:Vec<Item_>) -> Self {
		Self::new3(a, "")
	}
	pub fn new3(a_:Vec<Item_>, rem:&str) -> Self {
		Self {a_, rem_:rem.to_string()}
	}
	
	pub fn add__(&self, i2:&Item_, a:&mut A_, argv:&mut Vec<String>) {
		let cnt = if self.can_1__(i2) {
			i2.argc_ - 1
		} else {
			i2.argc_
		};
		for _ in 0..cnt {
			if let Some(i) = a.next() {
				argv.push(i.to_string());
			} else {
				break;
			}
		}
	}
	
	fn can_1__(&self, i2:&Item_) -> bool {
		i2.tag_.is_empty() && i2.argc_ > 0 && i2.argc_ < ARGC_Z_
	}

	/*pub fn for__(&self, a:&mut A_,
			cb:impl FnMut(&str, &Vec<String>, &Item_, u32, u32) -> i32,
			cb2:impl FnMut(&str) -> i32) -> i32 {
		if let Err((i, _)) = self.for3__(a, cb, cb2) {i} else {0}
	}*/
	pub fn for3__(&self, a:&mut A_,
			mut cb__:impl FnMut(&str, &Vec<String>, usize, &Item_, u32, u32) -> i32,
			mut cb2__:impl FnMut(&str) -> i32) -> Result_ {
		let mut argv = vec![];
		let mut no0 = 0;
		while let Some(i) = a.next() {
			let mut i3 = 0;
			argv.clear();
			let mut cbx__ = |i2, i3:&mut u32, no0, argv:&mut Vec<String>| -> i32 {
				let argc0 = argv.len();
				*i3 += 1;
				if *i3 == 1 {
					self.add__(i2, a, argv);
					loop {
						let mut argc = if i2.argc_ >= ARGC_Z_ {i2.argc_ - ARGC_Z_} else {i2.argc_}  + argc0;
						if argc == 0 {
							break;
						}
						if self.can_1__(i2) {
							argc -= 1
						}
						if argv.len() < argc {
							return ARG_NE_;
						}
						if true {
							break;
						}
					}
				}
				cb__(&i, &argv, argc0, i2, *i3, no0)
			};
			for i2 in self.a_.iter() {
				let i4 = i2.tag__(&i, &mut argv);
				if i4 >= 0 {
					let i5 = cbx__(i2, &mut i3, 0, &mut argv);
					if i5 != 0 {
						return self.err__(i5, i);
					}
				}
			}
			if i3 == 0 {
				no0 += 1;
				for i2 in self.a_.iter() {
					if i2.tag_.is_empty() {
						let i5 = cbx__(i2, &mut i3, no0, &mut argv);
						if i5 != 0 {
							return self.err__(i5, i);
						}
					}
				}
				if i3 == 0 {
					let i5 = cb2__(&i);
					if i5 != 0 {
						return self.err__(i5, i);
					}
				}
			}
		}
		Ok(())
	}

	fn err__(&self, i5:i32, i:String) -> Result_ {
		Err((i5, match i5 {
			HELP_ => self.help__(),
			_ => i
		}))
	}

	pub fn for2__(&self, a:&mut A_) {
		let mut argv = vec![];
		while let Some(i) = a.next() {
			let mut i3 = 0;
			let cbx__ = |i2:&Item_, argv:&Vec<String>| {
				if let Cb_::F(cb) = &i2.cb_ {
					cb(&argv, i2);
				}
			};
			for i2 in self.a_.iter() {
				let i4 = i2.tag__(&i, &mut argv);
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
			write!(&mut s, "\n{}", if i.tag_.is_empty() {
				if i.argc_ == ARGC_Z_ {"..."} else {".."}
			} else {&i.tag_}).unwrap();
			match i.typ_ {
				Typ_::Full => {}
				_ => write!(&mut s, " ({:?})", i.typ_).unwrap(),
			}
			match i.argc_ {
				0 => {}
				_ => if !(i.tag_.is_empty() && i.argc_ == ARGC_Z_) {
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
			if !i.rem_.is_empty() {
				write!(&mut s, " {}", i.rem_).unwrap();
			}
		}
		writeln!(&mut s).unwrap();
		s
	}
}
