use super::*;
use super::super::*;
use std::sync::Mutex;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Copy, Clone)]
pub struct Opt_ {
	pub jvhao_:bool,
	pub dunhao_:bool,
	pub guandao_du_:bool,
	pub guandao_jie_:bool,
}
impl Opt_ {
	pub fn new() -> Self {
		Opt_ {jvhao_:false, dunhao_:false, guandao_du_:false, guandao_jie_:false}
	}
}

pub fn hello__(codes:&List_, gd:Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
	if w.dbg_.arg_ {
		w.dbg_.arg__(&q.borrow().args_);
	}
	if w.dbg_.lc_ {
		println!()
	}
	let v = q.borrow().args_.to_vec__();
	q.borrow_mut().args2_ = v;
	codes.hello__(gd, q, w, ret)?;
	if w.dbg_.lc_ {
		println!()
	}
	ok__()
}

pub fn for_i__(i:&BI_, mut fkw:impl FnMut(&keyword_::Item_, i32), mut fa:impl FnMut(&List_, i32)) {
	fkw(&i.kw__(), 1);
	if let Some(a) = i.a__() {
		fa(a, 1);
	}
	if let Some(kw) = i.kw2__() {
		fkw(&kw, 2);
	}
	if let Some(a) = i.a2__() {
		fa(a, 2);
	}
	if let Some(kw) = i.kw3__() {
		fkw(&kw, 3);
	}
	if let Some(a) = i.a3__() {
		fa(a, 3);
	}
}

pub type BI_ = Box<dyn Item_>;
pub type OL_ = Option<List_>;
pub type ORL_<'a> = Option<&'a List_>;

pub type SR3_<'a> = &'a Mutex<&'a mut String>;

pub trait Item_ {
	fn kw__(&self) -> &keyword_::Item_;
	fn kw2__(&self) -> keyword_::ORI_ {None}
	fn kw3__(&self) -> keyword_::ORI_ {None}
	fn s__(&self, ret:&mut String, _w:&World_) {
		ret.push_str(&self.kw__().s_);
	}
	fn s2__(&self) -> &str {""}
	fn s3__(&self, ret:SR3_, w:&World_) {
		self.s__(&mut ret.lock().unwrap(), w);
	}
	fn add__(&mut self, _a:List_) -> Result2_ {ok__()}
	fn add2__(&mut self, _a:List_) -> Result2_ {ok__()}
	fn add3__(&mut self, _a:List_) -> Result2_ {ok__()}
	fn a__(&self) -> ORL_ {None}
	fn a2__(&self) -> ORL_ {None}
	fn a3__(&self) -> ORL_ {None}
	fn hello__(&self, gd:Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_;
}

pub struct Item1_ {
	kw_:keyword_::RI_,
	kw2_:keyword_::ORI_,
	kw3_:keyword_::ORI_,
}

impl Item1_ {
	pub fn new(kw:&keyword_::RI_) -> Self {
		Self {kw_:kw.clone(), kw2_:None, kw3_:None}
	}
	pub fn new2(kw:&keyword_::RI_, kw2:&keyword_::RI_) -> Self {
		Self {kw_:kw.clone(), kw2_:Some(kw2.clone()), kw3_:None}
	}
	pub fn new3(kw:&keyword_::RI_, kw2:&keyword_::RI_, kw3:&keyword_::RI_) -> Self {
		Self {kw_:kw.clone(), kw2_:Some(kw2.clone()), kw3_:Some(kw3.clone())}
	}

	pub fn s2__(&self, s:&str, ret:&mut String) {
		ret.push_str(&self.kw_.s_);
		ret.push_str(s);
		ret.push_str(&t_::or__(&self.kw2_).s_);
	}
	
	/*pub fn split__(args:&mut result_::List_, mut src:impl FnMut(result_::RRI_) -> Result2_) -> Result2_ {
		let mut cnt = 0;
		for i in args.iter() {
			cnt += 1;
			if i.borrow().dunhao__() {
				break
			}
		}
		for _ in 0..cnt {
			src(args.remove(0))?;
		}
		ok__()
	}*/
	
	pub fn split2_0__() -> usize {core::usize::MAX - 1}
	pub fn split2_1__(a:&code_::List_, sp:&mut usize) {
		for (idx, i) in a.iter().enumerate() {
			if i.kw__().id_ == keyword_::Id_::Dunhao {
				*sp = idx;
				break
			}
		}
	}
	pub fn split2_2__(a:code_::ORL_, sp:usize, s: &mut String, mut frem:impl FnMut(&str) -> bool,
			gd:Opt_, q:qv_::T_, w:&mut World_, ret2: &mut result_::List_) -> Result2_ {
		let a = t_::o__(&a);
		let mut ret4 = result_::List_::new();
		let mut idx = 0;
		a.hello2__(&mut idx, sp, gd, q.clone(), w, &mut ret4)?;
		for i in ret4.iter() {
			let i = i.borrow();
			i.s__(s);
			for rem in &i.rem_ {
				if frem(rem) {
					continue
				}
				return w.no_rem2__(&rem)
			}
		}
		idx = sp + 1;
		a.hello2__(&mut idx, core::usize::MAX, gd, q.clone(), w, ret2)
	}
	
	pub fn qv4rem__(&self, rems:&[String], mut shou:impl FnMut(&str) -> bool, q:qv_::T_, w:&mut World_)
			-> Result<Option<qv_::T_>, Result2_> {
		let mut q2 = Some(q);
		for i2 in rems {
			let rem = i2.as_str();
			match rem {
				"顶" =>
					q2 = Some(w.top_q_.clone()),
				"上" => {
					q2 = match &q2.unwrap().borrow_mut().up_ {
						Some(q3) => Some(q3.clone()),
						None => return Err(result2_::err2__("无上")),
					}
				}
				_ => {
					let ret2 = qv_::for__(q2.clone().unwrap(), w, |q3, _| {
						if q3.borrow().name_ == rem {
							q2 = Some(q3.clone());
							Some(())
						} else {None}
					});
					if ret2.is_none() && !shou(i2) {
						return Err(result2_::err__([&w.text__(i2), "注解不支持"].concat()))
					}
				}
			}
		}
		Ok(q2)
	}
	
	pub fn err__(&self, s:&str) -> Result2_ {
		result2_::err__([&self.kw__().s_, s].concat())
	}
	pub fn chk_empty__(&self, a:&List_, s:&str) -> Result2_ {
		if a.a_.is_empty() {self.err__(s)} else {ok__()}
	}
}

impl Item_ for Item1_ {
	fn kw__(&self) -> &keyword_::Item_ {&self.kw_}
	fn kw2__(&self) -> keyword_::ORI_ {self.kw2_.clone()}
	fn kw3__(&self) -> keyword_::ORI_ {self.kw3_.clone()}
	fn hello__(&self, _gd:Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		if let keyword_::Id_::Brkpoint = &self.kw_.id_ {
			let _ret2:Option<()> = qv_::for__(q, w, |q, w| {
				let q = q.borrow();
				if w.dbg_.lc_ {
					lc3__!("({})", q.src_);
				}
				None
			});
			#[allow(non_snake_case)]
			#[allow(unused_variables)]
			let o_X_o = true;
		} else {
			ret.add2__(self.kw_.clone());
		}
		ok__()
	}
}

pub struct List_ {
	a_:Vec<BI_>,
}

impl Deref for List_ {
	type Target = Vec<BI_>;
	fn deref(&self) -> &Self::Target {&self.a_}
}
impl DerefMut for List_ {
	fn deref_mut(&mut self) -> &mut Vec<BI_> {&mut self.a_}
}

impl List_ {
	pub fn new() -> Self {
		Self {a_:vec![]}
	}
	pub fn add__(&mut self, i:BI_) {
		self.a_.push(i);
	}
	#[allow(dead_code)]
	pub fn add2__(&mut self, i:impl Item_ + 'static) {
		self.a_.push(Box::new(i));
	}

	fn s3_i__(i:&BI_, ret:SR3_, w:&World_) {
		i.s3__(ret, w);
		for_i__(i, |kw, n| {
			if n > 1 {
				ret.lock().unwrap().push_str(&kw.s_);
			}
		}, |a, _n| a.s3__(&ret, w));
	}
	fn s3__(&self, ret:SR3_, w:&World_) {
		for i in &self.a_ {
			Self::s3_i__(i, ret, w);
		}
	}
	
	pub fn hello__(&self, gd:Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		self.hello2__(&mut 0, core::usize::MAX, gd, q, w, ret)
	}
	pub fn hello2__(&self, idx:&mut usize, end:usize,
			gd:Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		let a = &self.a_;
		let end = if end > a.len() {a.len()} else {end};
		let mut ret2 = self.z2__(idx, end, None, gd, q, w, ret);
		if let Err((_, _, s)) = &mut ret2 {
			s.push('\n');
			s.push_str(&w.kws_.begin_text_.s_);
			let ms = Mutex::new(s);
			for idx in *idx..end {
				Self::s3_i__(&a[idx], &ms, w)
			}
			ms.lock().unwrap().push_str(&w.kws_.end_text_.s_);
		}
		ret2
	}
	fn z2__(&self, idx:&mut usize, end:usize, v_dunhao3:Option<Vec<(usize, usize, usize)>>,
			gd:Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		let a = &self.a_;
		while *idx < end {
			let i = &a[*idx];
			if w.dbg_.lc_ {
				w.dbg_.lc__(i.as_ref(), w);
			}
			let id = &i.kw__().id_;
			match id {
				keyword_::Id_::Undef => {
					let cs:Vec<char> = i.s2__().chars().collect();
					let mut has = false;
					if let Some(v_dunhao3) = &v_dunhao3 {
						has = true;
						let mut first = true;
						let mut i2 = 0;
						let mut idx5 = *idx;
						for i in v_dunhao3 {
							let mut has2 = false;
							self.z5__(&cs, &mut has, idx, &mut idx5, end, i, &mut first, &mut has2,
								gd, q.clone(), w, ret)?;
							if has2 {
								i2 += 1
							}
						}
						if i2 > 0 {
							if idx5 > *idx {
								*idx = idx5;
							} else {
								*idx += 1
							}
							continue
						}
					}
					self.z4__(&cs, &mut 0, cs.len(), &mut has, idx, end, gd, q.clone(), w, ret)?;
					if has {
						continue
					}
					i.hello__(gd, q.clone(), w, ret)?;
				}
				keyword_::Id_::Jvhao => {
					if gd.jvhao_ {
						break
					}
					i.hello__(gd, q.clone(), w, ret)?;
				}
				keyword_::Id_::Dunhao => {
					if gd.dunhao_ {
						*idx += 1;
						break
					}
					i.hello__(gd, q.clone(), w, ret)?;
				}
				_ => {
					i.hello__(gd, q.clone(), w, ret)?;
				}
			}
			*idx += 1
		}
		ok__()
	}
	fn z5__(&self, cs:&[char], has:&mut bool, idx:&usize, idx5:&mut usize, end:usize,
			dunhao3:&(usize, usize, usize), first:&mut bool, has2:&mut bool,
			gd:Opt_, q:qv_::T_, w:&mut World_, ret2:&mut result_::List_) -> Result2_ {
		let (idx4, from4, end4) = dunhao3;
		if idx4 == idx {
			//lc3__!("\n{} {},{},{}",idx,idx4, from4, end4);
			let mut idx2 = *from4;
			*idx5 = *idx;
			if *first {
				*first = false
			} else {
				w.dunhao__(ret2);
			}
			self.z4__(cs, &mut idx2, *end4, has, idx5, end, gd, q, w, ret2)?;
			*has2 = true
		} else {
			*has2 = false
		}
		ok__()
	}
	#[allow(clippy::cognitive_complexity)]
	#[allow(clippy::too_many_arguments)]
	fn z4__(&self, cs:&[char], idx2:&mut usize, end2:usize, has:&mut bool, idx:&mut usize, end:usize,
			gd:Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		let mut paichu_def = vec![];
		'l1: while *idx2 < end2 {
			if let Some((idx3, len, def)) = qv_::find_def__(&cs, *idx2, end2, &paichu_def, q.clone(), w) {
				if w.dbg_.lc_ {
					w.dbg_.def__(&def)
				}
				let def = def.borrow();
				let mut v_dunhao3 = vec![];
				if let Some(argnames) = def.names__() {
					if argnames.has_ge_ {
						let a = &self.a_;
						let mut i_argname = 0;
						let f__ = |cs:&[char], from, i_argname:&mut usize| -> Option<(usize, usize)> {
							if *i_argname < argnames.len() {
								let argname = &argnames[*i_argname];
								//lc5__!("({} {}{})", i_argname, argname.s_, argname.is_ge_);
								if argname.is_ge_ {
									let mut idx = from;
									while idx < end2 {
										if let Some(len2) = t_::with__(cs, &argname.s_, idx) {
											//lc6__!("({}匹)", idx);
											*i_argname += 2;
											return Some((idx, len2))
										}
										idx += 1
									}
								}
							}
							None
						};
						{
							let mut idx2 = *idx2 + len;
							loop {
								if let Some((idx4, len)) = f__(cs, idx2, &mut i_argname) {
									v_dunhao3.push((*idx, idx2, idx4));
									idx2 = idx4 + len;
								} else if i_argname == 0 {
									i_argname += 1
								} else {
									if idx2 < end2 {
										v_dunhao3.push((*idx, idx2, end2));
									}
									break
								}
							}
						}
						let mut idx = *idx + 1;
						while idx < end {
							let i = &a[idx];
							match i.kw__().id_ {
								keyword_::Id_::Undef => {
									//lc3__!("\n{}", i.s2__());
									let mut idx2 = 0;
									let cs2:Vec<char> = i.s2__().chars().collect();
									while let Some((idx4, len)) = f__(&cs2, idx2, &mut i_argname) {
										v_dunhao3.push((idx, idx2, idx4));
										idx2 = idx4 + len;
									}
									if idx2 > 0 {
										v_dunhao3.push((idx, idx2, cs2.len()));
									}
								}
								keyword_::Id_::Dunhao => {
									if i_argname < argnames.len() {
										let argname = &argnames[i_argname];
										if argname.is_ge_ {
											paichu_def.push(def.arg0__().to_string());
											continue 'l1
										}
									} else {
										break
									}
									i_argname += 1;
								}
								keyword_::Id_::Jvhao => {
									break
								}
								_ => {}
							}
							idx += 1
						}
						if i_argname < argnames.len() {
							paichu_def.push(def.arg0__().to_string());
							continue 'l1
						}
					}
				}
				paichu_def.clear();
				*has = true;
				if v_dunhao3.is_empty() {
					while *idx2 < idx3 {
						ret.add__(cs[*idx2]);
						*idx2 += 1
					}
				}
				*idx2 += len;
				if def.simp__() {
					ret.add__(def.val__())
				} else {
					let mut codes:codes_cache_::OI_<code_::List_> = None;
					w.codes_cache__(def.val__(), |i| codes = Some(i.unwrap().clone()))?;
					let codes = codes.unwrap();
					
					let mut q2 = qv_::Qv_::new5(def.arg0__(), def.names__(), Some(q.clone()));
					if def.argc__() > 0 {
						if v_dunhao3.is_empty() {
							self.z4__(cs, idx2, end2, has, idx, end, gd, q.clone(), w, &mut q2.args_)?;
						} else {
							let mut first = true;
							let mut idx5 = *idx + 1;
							while !v_dunhao3.is_empty() {
								let mut has2 = false;
								self.z5__(cs, has, idx, &mut idx5, end, &v_dunhao3[0], &mut first, &mut has2,
									gd, q.clone(), w, &mut q2.args_)?;
								if has2 {
									v_dunhao3.remove(0);
								} else {
									break
								}
							}
							*idx2 = end2;
							*idx = idx5;
						}
						let mut gd2 = gd;
						gd2.jvhao_ = true;
						self.z2__(idx, end, Some(v_dunhao3), gd2, q.clone(), w, &mut q2.args_)?;
					};
					hello__(&codes, gd, qv_::t__(q2), w, ret)?;
				}
			} else {
				if *has {
					ret.add__(cs[*idx2]);
				}
				*idx2 += 1
			}
		}
		if *has {
			*idx += 1;
		}
		ok__()
	}
}
