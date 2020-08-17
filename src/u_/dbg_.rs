use super::{*, super::{as_ref__}};

#[macro_export]
macro_rules! p__ {
	($($arg:tt)*) => (
		print!($($arg)*);
		t_::pf__();
	)
}

/*
白色（7）是关键字色，背景不同区分用途
蓝色（4）是输入的参数色
红色（1）是结果，背景区分阶段
黑色（0）就是输出的一般色
绿色（2）黄色（3）粉色（5）青色（6）
*/
pub fn begin_ansi_s__(id:&keyword_::Id_) -> &str {
	match id {
		keyword_::Id_::Undef => "34",
		keyword_::Id_::U2 => "32",
		keyword_::Id_::U3 => "33",
		keyword_::Id_::U4 => "34",
		keyword_::Id_::U5 => "35",
		keyword_::Id_::U6 => "36",
		keyword_::Id_::U7 => "31",
		keyword_::Id_::U9 => "37;42",
		keyword_::Id_::U10 => "37;40",
		keyword_::Id_::Jvhao | keyword_::Id_::EndBlock => "37;41",
		_ => "37;44"
	}
}
pub fn begin_ansi__(s:&str) {
	p__!("\x1b[0;{}m", s);
}
pub fn end_ansi__() {
	p__!("\x1b[0m");
}

#[macro_export]
macro_rules! lc_kw__ {
	($id:expr, $($arg:tt)*) => (
		#[allow(unused_imports)]
		use crate::u_::dbg_::*;
		begin_ansi__(begin_ansi_s__($id));
		p__!($($arg)*);
		end_ansi__();
	)
}
#[macro_export] macro_rules! lc__ {($($arg:tt)*) => (lc_kw__!(&keyword_::Id_::U3, $($arg)*);)}
#[macro_export] macro_rules! lc2__ {($($arg:tt)*) => (lc_kw__!(&keyword_::Id_::U2, $($arg)*);)}
#[macro_export] macro_rules! lc3__ {($($arg:tt)*) => (lc_kw__!(&keyword_::Id_::U6, $($arg)*);)}
#[macro_export] macro_rules! lc4__ {($($arg:tt)*) => (lc_kw__!(&keyword_::Id_::U4, $($arg)*);)}
#[macro_export] macro_rules! lc5__ {($($arg:tt)*) => (lc_kw__!(&keyword_::Id_::U5, $($arg)*);)}
#[macro_export] macro_rules! lc6__ {($($arg:tt)*) => (lc_kw__!(&keyword_::Id_::U7, $($arg)*);)}

#[derive(Default, Clone, Debug)]
pub struct Dbg_ {
	pub tree_:bool,
	pub arg_:bool,
	pub path_:bool,
	pub lc_:bool,
	pub par_lc_:bool,
	pub parbp_:String,
	pub bp_:String,
	pub expl_:bool,
	pub if_:bool,
}

impl Dbg_ {
	pub fn tree__(&self, l:&code_::List_, w:&World_) {
		println!();
		let mut wei:[char;64] = ['*';64];
		self.tree2__(l, 0, &mut wei, false, w)
	}
	fn tree2__(&self, l:&code_::List_, suojin:usize, wei:&mut [char], is_end2:bool, w:&World_) {
		let len = l.len();
		let mut i2 = 0;
		let end__ = |kw:keyword_::ORI_, i2| {
			if kw.is_some() {false} else {i2 == len}
		};
		let end2__ = |is_end, i2| {
			if suojin == 0 {i2 == len && is_end} else {is_end2}
		};
		for i in l.iter() {
			i2 += 1;
			let is_end = end__(as_ref__!(i).kw2__(), i2);
			let is_end2 = end2__(is_end, i2);
			self.tree2_i__(i, suojin, wei, is_end, i2 == 1, is_end2, w);
			if let Some(a) = as_ref__!(i).a__() {
				self.tree2__(a, suojin + 1, wei, is_end2, w)
			}

			let is_end = end__(as_ref__!(i).kw3__(), i2);
			let is_end2 = end2__(is_end, i2);
			if let Some(kw) = as_ref__!(i).kw2__() {
				self.tree_line__(" ", &kw.s_, suojin, wei, is_end, false, is_end2)
			}
			if let Some(a) = as_ref__!(i).a2__() {
				self.tree2__(a, suojin + 1, wei, is_end2, w)
			}

			if let Some(kw) = as_ref__!(i).kw3__() {
				self.tree_line__(" ", &kw.s_, suojin, wei, i2 == len, false, is_end2)
			}
			if let Some(a) = as_ref__!(i).a3__() {
				self.tree2__(a, suojin + 1, wei, is_end2, w)
			}
		}
	}
	fn tree_line__(&self, sp3:&str, s:&str, suojin:usize, wei:&mut [char],
			is_end:bool, is_begin:bool, is_end2:bool) {
		p__!("{:2}{}{}{} ", suojin, t_::b__(is_end2), t_::b__(is_begin), t_::b__(is_end));
		self.begin_ansi_kw__(&keyword_::Id_::U2);
		let sp1:char =
				 if suojin == 0 && is_end && is_begin	{'<'}
			else if suojin == 0 && is_begin				{'/'}
			else if suojin == 0 && (is_end || is_end2)	{'\\'}
			else if suojin == 0							{'|'}
			else if is_end2 && is_end					{'\\'}
			else if is_end2								{'|'}
			else if is_end								{'\\'}
			else										{'|'};
		wei[suojin] = sp1;
		let mut s2 = String::new();
		for i in 0..suojin {
			s2.push(match wei[i] {
				'\\' | '<' => ' ',
				_ => '|'
			});
			s2.push_str("  ");
		}
		p__!("{}{}{}{}", s2, sp1, sp3, s);
		end_ansi__();
		println!();
	}
	fn tree2_i__(&self, i:&code_::I_, suojin:usize,  wei:&mut [char], is_end:bool, is_begin:bool, is_end2:bool, w:&World_) {
		let mut s = String::new();
		as_ref__!(i).s__(&mut s, w);
		self.tree_line__("-", &s, suojin, wei, is_end, is_begin, is_end2)
	}
	
	pub fn arg__(&self, a:&result_::List_) {
		if !a.is_empty() {
			println!();
		}
		let mut i2 = 1;
		let mut i3 = 0;
		for (idx, i) in a.iter().enumerate() {
			let mut s = String::new();
			s.push_str(&format!("({:2}", idx));
			if let result_::Val_::K(_) = &*as_ref__!(as_ref__!(i).val_) {
				i2 += 1;
				i3 = 0;
				s.push(')');
				s.push_str(&"-".repeat(4))
			} else {
				i3 += 1;
				if i3 == 1 {
					s.push_str(&format!("/{:<2}) ", i2))
				} else {
					s.push(')');
					s.push_str(&" ".repeat(4))
				}
			}
			as_ref__!(i).s2__(&mut s, true, true, true, true);
			self.arg3__(&s, false);
			for rem in &as_ref__!(i).rem_ {
				self.rem__(rem);
			}
			println!();
		}
	}
	pub fn arg2__(&self, s:&str) {
		self.arg3__(s, true)
	}
	pub fn arg3__(&self, s:&str, ln:bool) {
		self.begin_ansi_kw__(&keyword_::Id_::U4);
		p__!("{}", s);
		end_ansi__();
		if ln {
			println!();
		}
	}

	pub fn lc__(&self, i:&code_::I_, w:&World_) {
		let mut s = String::new();
		as_ref__!(i).s__(&mut s, w);
		self.lc2__(as_ref__!(i).kw__(), &s);
	}
	pub fn lc2__(&self, kw:&keyword_::Item_, s:&str) {
		self.lc3__(&kw.id_, s)
	}
	fn lc3__(&self, id:&keyword_::Id_, s:&str) {
		self.begin_ansi_kw__(id);
		p__!("{}", s);
		end_ansi__();
	}
	pub fn lc_kw__(&self, kw:&keyword_::Item_) {
		self.lc3__(&kw.id_, &kw.s_)
	}

	pub fn def__(&self, i:&def_::I_) {
		self.begin_ansi_kw__(&keyword_::Id_::U10);
		let i = as_ref__!(i);
		let name = i.arg0__();
		let argc = i.argc__();
		p__!("{}({})", name, if argc > 99 {99} else {argc});
		end_ansi__();
	}
	
	pub fn rem__(&self, rem:&str) {
		p__!(" ");
		self.begin_ansi_kw__(&keyword_::Id_::U9);
		p__!("{}", rem);
		end_ansi__();
	}

	fn begin_ansi_kw__(&self, id:&keyword_::Id_) {
		begin_ansi__(begin_ansi_s__(id));
	}
}
