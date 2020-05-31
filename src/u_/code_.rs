use super::{*, super::{as_ref__, cfg_if}};
use std::{sync::Mutex, ops::{Deref, DerefMut}};

#[derive(Copy, Clone, Default)]
pub struct Opt_ {
	pub jvhao_:bool,
	pub dunhao_:bool,
	pub guandao_du_:bool,
	pub guandao_jie_:bool,
	pub undef_eq_text_:bool,
	pub vals_:bool,
}

pub fn hello__(codes:&List_, gd:Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
	if wm.dbg_.arg_ {
		wm.dbg_.arg__(&as_ref__!(q).args_);
	}
	codes.hello__(gd, q, w, wm, ret)?;
	ok__()
}

pub fn for_i__(i:&I_, mut fkw:impl FnMut(&keyword_::Item_, i32), mut fa:impl FnMut(&List_, i32)) {
	fkw(&as_ref__!(i).kw__(), 1);
	if let Some(a) = as_ref__!(i).a__() {
		fa(a, 1);
	}
	if let Some(kw) = as_ref__!(i).kw2__() {
		fkw(&kw, 2);
	}
	if let Some(a) = as_ref__!(i).a2__() {
		fa(a, 2);
	}
	if let Some(kw) = as_ref__!(i).kw3__() {
		fkw(&kw, 3);
	}
	if let Some(a) = as_ref__!(i).a3__() {
		fa(a, 3);
	}
}

cfg_if! {
	if #[cfg(feature = "thread")] {
		pub type I_ = Rc_<RefCell_<dyn Item_ + Send + Sync>>;

		pub fn i__(i:impl Item_ + Send + Sync + 'static) -> I_ {
			Rc_::new(RefCell_::new(i))
		}
		pub fn oi__(i:impl Item_ + Send + Sync + 'static) -> OI_ {
			Some(Rc_::new(RefCell_::new(i)))
		}
	} else {
		pub type I_ = Rc_<RefCell_<dyn Item_>>;

		pub fn i__(i:impl Item_ + 'static) -> I_ {
			Rc_::new(RefCell_::new(i))
		}
		pub fn oi__(i:impl Item_ + 'static) -> OI_ {
			Some(Rc_::new(RefCell_::new(i)))
		}
	}
}
pub type OI_ = Option<I_>;
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
	fn hello__(&self, gd:Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_;
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
	
	pub fn split2_0__() -> usize {core::usize::MAX - 1}
	pub fn split2_1__(a:&code_::List_, sp:&mut usize) {
		for (idx, i) in a.iter().enumerate() {
			if as_ref__!(i).kw__().id_ == keyword_::Id_::Dunhao {
				*sp = idx;
				break
			}
		}
	}
	pub fn split2_2__(a:code_::ORL_, sp:usize, s: &mut String, mut frem:impl FnMut(&str) -> Result2_,
			gd:Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret2: &mut result_::List_) -> Result2_ {
		let a = t_::o__(&a);
		let mut ret4 = result_::List_::new();
		let mut idx = 0;
		a.hello2__(&mut idx, sp, gd, q.clone(), w.clone(), wm, &mut ret4)?;
		for i in ret4.iter() {
			let i = as_ref__!(i);
			i.s__(s);
			for rem in &i.rem_ {
				frem(rem)?;
			}
		}
		idx = sp + 1;
		a.hello2__(&mut idx, core::usize::MAX, gd, q, w, wm, ret2)
	}
	
	pub fn qv4rem_1__(&self, rem:&str, mut shou:impl FnMut(&str) -> bool, q:qv_::T_, w:world_::T_)
			-> Result<Option<qv_::T_>, Result2_> {
		let mut q2 = Some(q);
		match rem {
			"顶" =>
				q2 = Some(as_ref__!(w).top_q_.clone()),
			"上" => {
				q2 = match &as_ref__!(q2.unwrap()).up_ {
					Some(q3) => Some(q3.clone()),
					None => return Err(result2_::err2__("无上")),
				}
			}
			_ => {
				let ret2 = qv_::for__(q2.clone().unwrap(), w.clone(), |q3, _, _| {
					if as_ref__!(q3).name_.contains(&rem.to_string()) {
						q2 = Some(q3.clone());
						Some(())
					} else {None}
				});
				if ret2.is_none() && !shou(rem) {
					return Err(result2_::err__([&as_ref__!(w).text__(rem), "注解不支持"].concat()))
				}
			}
		}
		Ok(q2)
	}
	pub fn qv4rem__(&self, rems:&[String], mut shou:impl FnMut(&str) -> bool, q:qv_::T_, w:world_::T_)
			-> Result<Option<qv_::T_>, Result2_> {
		let mut q2 = Some(q);
		for rem in rems {
			match self.qv4rem_1__(rem.as_str(), &mut shou, q2.clone().unwrap(), w.clone()) {
				Ok(q3) => q2 = q3,
				e => return e
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
	fn hello__(&self, _gd:Opt_, _q:qv_::T_, _w:world_::T_, _wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		if let keyword_::Id_::Brkpoint = &self.kw_.id_ {
			#[allow(non_snake_case)]
			#[allow(unused_variables)]
			let o_X_o = true;
		} else {
			ret.add2__(self.kw_.clone());
		}
		ok__()
	}
}

#[derive(Default, Clone)]
pub struct List_ {
	a_:Vec<I_>,
}

impl Deref for List_ {
	type Target = Vec<I_>;
	fn deref(&self) -> &Self::Target {&self.a_}
}
impl DerefMut for List_ {
	fn deref_mut(&mut self) -> &mut Vec<I_> {&mut self.a_}
}

impl List_ {
	pub fn new() -> Self {
		Self {a_:vec![]}
	}
	pub fn add__(&mut self, i:I_) {
		self.a_.push(i);
	}
cfg_if! {
	if #[cfg(feature = "thread")] {

	pub fn add2__(&mut self, i:impl Item_ + Send + Sync + 'static) {
		self.add__(i__(i));
	}

	}
}
	fn s3_i__(i:&I_, ret:SR3_, w:&World_) {
		as_ref__!(i).s3__(ret, w);
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
	
	pub fn hello__(&self, gd:Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		self.hello2__(&mut 0, core::usize::MAX, gd, q, w, wm, ret)
	}
	pub fn hello2__(&self, idx:&mut usize, end:usize,
			gd:Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		let a = &self.a_;
		let end = if end > a.len() {a.len()} else {end};
		let mut ret2 = self.z2__(idx, end, None, gd, q, w.clone(), wm, ret);
		if let Err((_, _, s)) = &mut ret2 {
			if *idx < end {
				s.push('\n');
				let kws = &as_ref__!(w).kws_;
				s.push_str(&kws.begin_text_.s_);
				let ms = Mutex::new(s);
				for idx in *idx..end {
					Self::s3_i__(&a[idx], &ms, &as_ref__!(w))
				}
				ms.lock().unwrap().push_str(&kws.end_text_.s_);
			}
		}
		ret2
	}
	fn z2__(&self, idx:&mut usize, end:usize, v_dunhao3:Option<Vec<(usize, usize, usize)>>,
			gd:Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		let a = &self.a_;
		while *idx < end {
			let i = &a[*idx];
			if wm.dbg_.lc_ {
				wm.dbg_.lc__(i, &as_ref__!(w));
			}
			match as_ref__!(i).kw__().id_ {
				keyword_::Id_::Undef => {
					if !gd.undef_eq_text_ {
						let cs:Vec<char> = as_ref__!(i).s2__().chars().collect();
						let mut has = false;
						if let Some(v_dunhao3) = &v_dunhao3 {
							has = true;
							let mut first = true;
							let mut i2 = 0;
							let mut idx5 = *idx;
							for i in v_dunhao3 {
								let mut has2 = false;
								self.z5__(&cs, &mut has, idx, &mut idx5, end, i, &mut first, &mut has2,
									gd, q.clone(), w.clone(), wm, ret)?;
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
						self.z4__(&cs, &mut 0, cs.len(), &mut has, idx, end, gd, q.clone(), w.clone(), wm, ret)?;
						if has {
							continue
						}
					}
					as_ref__!(i).hello__(gd, q.clone(), w.clone(), wm, ret)?;
				}
				keyword_::Id_::Jvhao => {
					if gd.jvhao_ {
						break
					}
					as_ref__!(i).hello__(gd, q.clone(), w.clone(), wm, ret)?;
				}
				keyword_::Id_::Dunhao => {
					if gd.dunhao_ {
						*idx += 1;
						break
					}
					as_ref__!(i).hello__(gd, q.clone(), w.clone(), wm, ret)?;
				}
				_ => {
					as_ref__!(i).hello__(gd, q.clone(), w.clone(), wm, ret)?;
				}
			}
			*idx += 1
		}
		ok__()
	}
	fn z5__(&self, cs:&[char], has:&mut bool, idx:&usize, idx5:&mut usize, end:usize,
			dunhao3:&(usize, usize, usize), first:&mut bool, has2:&mut bool,
			gd:Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret2:&mut result_::List_) -> Result2_ {
		let (idx4, from4, end4) = dunhao3;
		if idx4 == idx {
			//lc3__!("\n{} {},{},{}",idx,idx4, from4, end4);
			let mut idx2 = *from4;
			*idx5 = *idx;
			if *first {
				*first = false
			} else {
				as_ref__!(w).dunhao__(ret2);
			}
			self.z4__(cs, &mut idx2, *end4, has, idx5, end, gd, q, w, wm, ret2)?;
			*has2 = true
		} else {
			*has2 = false
		}
		ok__()
	}
	#[allow(clippy::cognitive_complexity)]
	#[allow(clippy::too_many_arguments)]
	fn z4__(&self, cs:&[char], idx2:&mut usize, end2:usize, has:&mut bool, idx:&mut usize, end:usize,
			gd:Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		let mut paichu_def = vec![];
		'l1: while *idx2 < end2 {
			if let Some((idx3, len, def)) = qv_::find_def__(&cs, *idx2, end2, &paichu_def, q.clone(), w.clone()) {
				if wm.dbg_.lc_ {
					wm.dbg_.def__(&def)
				}
				let def = as_ref__!(def);
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
										if let Some((len2, _)) = t_::with__(cs, &argname.s_, idx) {
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
							match as_ref__!(i).kw__().id_ {
								keyword_::Id_::Undef => {
									//lc3__!("\n{}", i.s2__());
									let mut idx2 = 0;
									let cs2:Vec<char> = as_ref__!(i).s2__().chars().collect();
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
				match &def.val_ {
					def_::Val_::Si(s) => ret.add__(s),
					_ => {
						let mut q2 = qv_::Qv_::new5(def.arg0__(), def.names__(), Some(q.clone()));
						if def.argc__() > 0 {
							let gd2 = Opt_ {jvhao_:true, ..gd};
							if v_dunhao3.is_empty() {
								self.z4__(cs, idx2, end2, has, idx, end, gd, q.clone(), w.clone(), wm, &mut q2.args_)?;
							} else {
								let mut first = true;
								let mut idx5 = *idx + 1;
								while !v_dunhao3.is_empty() {
									let mut has2 = false;
									self.z5__(cs, has, idx, &mut idx5, end, &v_dunhao3[0], &mut first, &mut has2,
										gd, q.clone(), w.clone(), wm, &mut q2.args_)?;
									if has2 {
										v_dunhao3.remove(0);
									} else {
										break
									}
								}
								*idx2 = end2;
								*idx = idx5;
							}
							self.z2__(idx, end, Some(v_dunhao3), gd2, q.clone(), w.clone(), wm, &mut q2.args_)?;
						};

						match &def.val_ {
							def_::Val_::S(src) => {
								let mut codes = None;
								wm.codes_cache__(&src, |_| {}, w.clone(), |i| codes = Some(i.unwrap().clone()))?;
								hello__(&codes.unwrap(), gd, qv_::t__(q2), w.clone(), wm, ret)?;
							}
							def_::Val_::F(f) => {
								q2.objs_ = Some(def.objs_.clone());
								f(&gd, qv_::t__(q2), w.clone(), wm, ret)?;
							}
							_ => {}
						}
					}
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
