use super::u_::*;
use super::u2_::*;
use std::process::{Command};

pub struct Item_ {
	super_:code_::Item1_,
	a_:code_::OL_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:code_::Item1_::new(&kws.exec_), a_:None}
	}

	fn exitcode__(&self, code:Option<i32>, w:&World_, ret:&mut result_::List_) {
		match code {
			Some(i) => {
				ret.add__(i);
				w.dunhao__(ret);
			}
			_ => {
				w.dunhao__(ret);
				ret.add__("Process terminated by signal");
			}
		}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn add__(&mut self, a:code_::List_) -> Result2_ {
		self.super_.chk_empty__(&a, "缺")?;
		self.a_ = Some(a);
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}
	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		let mut ret2 = result_::List_::new();
		t_::o__(&self.a_).hello__(gd, q.clone(), w, &mut ret2)?;
		let v2 = ret2.to_vec__();
		if v2.is_empty() {return self.super_.err__("空命令")};
		let mut args = str_::split__(&v2[0]);
		if args.is_empty() {return self.super_.err__("空命令")};
		let cmd = args.remove(0);
		if v2.len() > 1 {
			match Command::new(cmd).args(args).output() {
				Ok(output) => {
					self.exitcode__(output.status.code(), w, ret);
					let mut f__ = |i:usize, v: &[u8]| -> Result2_ {
						let mut codes:codes_cache_::OI_<code_::List_> = None;
						w.codes_cache__(&v2[i], |i| codes = Some(i.unwrap().clone()))?;
						let codes = codes.unwrap();

						if codes.is_empty() {
							match String::from_utf8(v.to_vec()) {
								Ok(s) => {
									ret.add__(s);
									w.dunhao__(ret);
								}
								Err(e) => {
									w.dunhao__(ret);
									ret.add__(e);
								}
							}
						} else {
							match std::str::from_utf8(v) {
								Ok(s) => {
									let v = str_::split2__(&s, "\r\n", true, true, false);
									let mut ret3 = result_::List_::new();
									for i in &v {
										let mut q2 = Qv_::new2(Some(q.clone()));
										q2.args_.add__(i);
										code_::hello__(&codes, gd, qv_::t__(q2), w, &mut ret3)?;
									}
									w.dunhao__(ret);
								}
								Err(e) => {
									w.dunhao__(ret);
									ret.add__(e);
								}
							}
						}
						ok__()
					};
					f__(1, &output.stdout)?;
					if v2.len() > 2 {
						f__(2, &output.stderr)?;
					}
				}
				Err(e) => {
					w.dunhao__(ret);
					w.dunhao__(ret);
					ret.add__(e);
				}
			}
		} else {
			match Command::new(cmd).args(args).status() {
				Ok(st) => {
					self.exitcode__(st.code(), w, ret);
				}
				Err(e) => {
					w.dunhao__(ret);
					ret.add__(e);
				}
			}
		}
		ok__()
	}
}
