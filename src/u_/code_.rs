use super::{*, super::{as_ref__, as_mut_ref__, cfg_if}};
#[cfg(debug_assertions)]
use super::super::{lc3__, lc2__, lc_kw__, p__, dbp2__};
#[cfg(debug_assertions)]
use super::super::{db_c__, lc6__};
use std::{sync::Mutex, ops::{Deref, DerefMut}};

mod item1_;
pub use item1_::*;
pub mod attr_;
pub type FA_ = attr_::I_;
mod env_;
pub type Env_ = env_::Env_;

#[derive(Copy, Clone, Default)]
pub struct Opt_ {
	pub jvhao_:bool,
	pub dunhao_:bool,
	pub guandao_du_:bool,
	pub guandao_jie_:&'static str,
	pub undef_eq_text_:bool,
	pub vals_:bool,
}

pub fn hello__(codes:&List_, env:&Env_) -> Result2_ {
	#[cfg(debug_assertions)]
	if as_ref__!(env.w).dbg_.arg_ {
		as_ref__!(env.w).dbg_.arg__(&as_ref__!(as_ref__!(env.q).args_));
	}
	codes.hello__(env)
}

pub fn for_i__(i:&I_, has1:bool, mut fkw:impl FnMut(&keyword_::Item_), mut fa:impl FnMut(&List_)) {
	let i = as_ref__!(i);
	if has1 {
		fkw(&i.kw__());
	}
	if let Some(a) = i.a__() {
		fa(a);
	}
	if let Some(kw) = i.kw2__() {
		fkw(&kw);
	}
	if let Some(a) = i.a2__() {
		fa(a);
	}
	if let Some(kw) = i.kw3__() {
		fkw(&kw);
	}
	if let Some(a) = i.a3__() {
		fa(a);
	}
}

cfg_if! {
	if #[cfg(feature = "thread")] {
		pub type I_ = Rc_<RefCell_<dyn Item_ + Send + Sync>>;

		pub fn i__(i:impl Item_ + Send + Sync + 'static) -> I_ {
			Rc_::new(RefCell_::new(i))
		}
		pub fn oi__(i:impl Item_ + Send + Sync + 'static) -> OI_ {
			Some(i__(i))
		}
	} else {
		pub type I_ = Rc_<RefCell_<dyn Item_>>;

		pub fn i__(i:impl Item_ + 'static) -> I_ {
			Rc_::new(RefCell_::new(i))
		}
		pub fn oi__(i:impl Item_ + 'static) -> OI_ {
			Some(i__(i))
		}
	}
}
pub type OI_ = Option<I_>;
pub type OL_ = Option<List_>;
pub type ORL_<'a> = Option<&'a List_>;
pub type A_ = Vec<I_>;

pub type MS_<'a> = Mutex<&'a mut String>;
type SR3_<'a> = &'a MS_<'a>;

pub trait Item_ {
	fn kw__(&self) -> &keyword_::Item_;
	fn kw2__(&self) -> keyword_::ORI_ {None}
	fn kw3__(&self) -> keyword_::ORI_ {None}
	fn s__(&self, ret:&mut String, _:&World_) {
		self.s_kw__(ret);
	}
	fn s2__(&self) -> &str {""}
	fn s3__(&self, ret:SR3_, w:&World_) {
		self.s__(&mut ret.lock().unwrap(), w);
	}
	fn s_kw__(&self, ret:&mut String) {
		ret.push_str(&self.kw__().s_);
	}
	fn add__(&mut self, _:List_) -> Result2_ {ok__()}
	fn add2__(&mut self, _:List_) -> Result2_ {ok__()}
	fn add3__(&mut self, _:List_) -> Result2_ {ok__()}
	fn a__(&self) -> ORL_ {None}
	fn a2__(&self) -> ORL_ {None}
	fn a3__(&self) -> ORL_ {None}
	fn hello__(&self, env:&Env_) -> Result2_;
}

#[derive(Default, Clone)]
pub struct List_ {
	pub a_:A_,
}

impl Deref for List_ {
	type Target = A_;
	fn deref(&self) -> &Self::Target {&self.a_}
}
impl DerefMut for List_ {
	fn deref_mut(&mut self) -> &mut A_ {&mut self.a_}
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
	pub fn s3_i__(i:&I_, ret:SR3_, w:&World_) {
		as_ref__!(i).s3__(ret, w);
		for_i__(i, false, |kw| ret.lock().unwrap().push_str(&kw.s_), |a| a.s3__(&ret, w));
	}
	fn s3__(&self, ret:SR3_, w:&World_) {
		for i in &self.a_ {
			Self::s3_i__(i, ret, w);
		}
	}
	
	pub fn hello__(&self, env:&Env_) -> Result2_ {
		self.hello2__(&mut 0, core::usize::MAX, env)
	}
	pub fn hello2__(&self, idx:&mut usize, end:usize, env:&Env_) -> Result2_ {
		let a = &self.a_;
		let end = if end > a.len() {a.len()} else {end};
		let mut ret2 = z2__(a, idx, end, env);
		if let Err((_, _, _, s)) = &mut ret2 {
			s.push('\n');
			let kws = &as_ref__!(env.w).kws_;
			s.push_str(&kws.begin_text_.s_);
			let ms = MS_::new(s);
			for idx in 0..=*idx {
				if idx >= a.len() {break}
				Self::s3_i__(&a[idx], &ms, &as_ref__!(env.w))
			}
			ms.lock().unwrap().push_str(&kws.end_text_.s_);
		}
		#[cfg(debug_assertions)]
		if as_ref__!(env.w).dbg_.lc_ {
			println!();
		}
		ret2
	}
}

fn cs__(i:&I_) -> Vec<char> {
	as_ref__!(i).s2__().chars().collect()
}
fn z2__(a:&A_, idx:&mut usize, end:usize, env:&Env_) -> Result2_ {
	while *idx < end {
		let i = &a[*idx];
		#[cfg(debug_assertions)]
		if as_ref__!(env.w).dbg_.lc_ {
			as_ref__!(env.w).dbg_.lc__(i, &as_ref__!(env.w));
		}
		match as_ref__!(i).kw__().id_ {
			keyword_::Id_::Undef => {
				if !env.gd.undef_eq_text_ {
					let cs = cs__(i);
					let mut has = false;
					z4__(a, &cs, &mut 0, cs.len(), &mut has, idx, end, env)?;
					if has {
						continue
					}
				}
				as_ref__!(i).hello__(env)?;
			}
			keyword_::Id_::Jvhao => {
				if env.gd.jvhao_ {
					break
				}
				if as_ref__!(env.w).cfg_.jvhao_dunhao_ {
					let b2 = if *idx > 0 {
						match &as_ref__!(a[*idx - 1]).kw__().id_ {
							keyword_::Id_::Dunhao |
							keyword_::Id_::BeginVar |
							keyword_::Id_::BeginText |
							keyword_::Id_::Undef => true,
							_ => {false}
						}
					} else {true};
					let b = if let Some(i2) = as_ref__!(env.ret).end__() {
						if !as_ref__!(i2).jvhao__() {
							true
						} else {false}
					} else {false};
					if b2 && b {
						as_ref__!(i).hello__(env)?;
					}
				}
			}
			keyword_::Id_::Dunhao => {
				if env.gd.dunhao_ {
					*idx += 1;
					break
				}
				as_ref__!(i).hello__(env)?;
			}
			_ => {
				as_ref__!(i).hello__(env)?;
				#[cfg(debug_assertions)]
				if dbp2__!("-tree-", env) {
					as_ref__!(env.w).dbg_.tree__(a, *idx, &as_ref__!(env.w));
				}
				#[cfg(debug_assertions)]
				if dbp2__!("-tree4-", env) {
					as_ref__!(env.w).dbg_.tree4__(a, *idx, &as_ref__!(env.w));
				}
			}
		}
		*idx += 1
	}
	ok__()
}
#[allow(clippy::cognitive_complexity)]
#[allow(clippy::too_many_arguments)]
fn z4__(a:&A_, cs:&[char], idx2:&mut usize, end2:usize, has:&mut bool, idx:&mut usize, end:usize, env:&Env_) -> Result2_ {
	#[cfg(debug_assertions)]
	if as_ref__!(env.w).dbg_.lch_ {
		lc3__!("\n{}-{},{})", idx2, end2, idx);
	}
	let mut paichu_def = vec![];
	let mut buf = String::new();
	'l1: while *idx2 < end2 {
		if let Some((idx3, len, def, def_q)) = qv_::find_def__(&cs, *idx2, end2, &paichu_def, env.q.clone(), env.w.clone()) {
			#[cfg(debug_assertions)]
			if as_ref__!(env.w).dbg_.lc_ {
				as_ref__!(env.w).dbg_.def__(&def)
			}
			let def = as_ref__!(def);
			let mut idx2_save = *idx2;
			*idx2 = idx3;
			let mut a2:A_ = vec![];
			let mut end2a = 0;
			if let Some(argnames) = def.names__() {
				if argnames.has_ge_ {
					#[cfg(debug_assertions)]
					{
						let w = as_ref__!(env.w);
						if w.dbg_.tree_ || db_c__!("-sparg3-", env) {
							w.dbg_.tree6__(&a, &w);
						}
					}
					let mut i_argname = 0;
					let f__ = |cs:&[char], from, i_argname:&mut usize, plus| -> Option<(usize, usize)> {
						if *i_argname < argnames.len() {
							let argname = &argnames[*i_argname];
							#[cfg(debug_assertions)]
							if db_c__!("-sparg-", env) {
								if from < cs.len() {
									lc2__!("({}{:?})", from, cs);
								}
								lc2__!("({} {}{} {})", from, argname.is_ge_, i_argname, argname.s_);
							}
							if argname.is_ge_ {
								let mut idx = from;
								while idx < end2 {
									#[cfg(debug_assertions)]
									if db_c__!("-sparg-", env) {
										lc2__!("{}", cs[idx]);
									}
									if let Some((len2, _)) = t_::with__(cs, &argname.s_, idx) {
										#[cfg(debug_assertions)]
										if db_c__!("-sparg-", env) {
											lc6__!("({}匹)", idx);
										}
										*i_argname += 2;
										return Some((idx, len2))
									}
									idx += 1
								}
								if plus {
									return Some((0, 0))
								}
							}
						}
						None
					};
					let dunhao3 = |idx2, idx4, len:usize, i:&I_, a2:&mut A_| {
						#[cfg(debug_assertions)]
						if db_c__!("-sparg-", env) {
							lc3__!("({},{}/{})", idx2, idx4, len);
						}

						let w = as_ref__!(env.w);
						if idx4 > idx2 {
							let s:String = cs__(i).get(idx2..idx4).unwrap().iter().collect();
							#[cfg(debug_assertions)]
							if db_c__!("-sparg-", env) {
								lc3__!("(\"{}\")", s);
							}
							a2.push(i__(super::super::undef_::Item_::new(&w.kws_, s)));
						}
						if idx4 < len {
							a2.push(i__(Item1_::new(&w.kws_.dunhao_)));
						}
					};
					let mut paichu = |idx2:&mut usize| {
						paichu_def.push(def.name__().to_string());
						*idx2 = idx2_save;
					};
					{
						let mut idx2m = *idx2 + len;
						loop {
							if let Some((idx4, len)) = f__(cs, idx2m, &mut i_argname, true) {
								if len == 0 {
									#[cfg(debug_assertions)]
									if db_c__!("-sparg-", env) {
										lc6__!("(b1)");
									}
									break;
								}
								dunhao3(idx2m, idx4, cs.len(), &a[*idx], &mut a2);
								idx2m = idx4 + len;
							} else {
								if i_argname <= def.backarg_ {
									i_argname += 1;
									continue;
								}
								if idx2m < end2 {
									dunhao3(idx2m, end2, cs.len(), &a[*idx], &mut a2);
								}
								break;
							}
						}
					}
					let mut idx = *idx + 1;
					#[cfg(debug_assertions)]
					if db_c__!("-sparg2-", env) {
						let w = as_ref__!(env.w);
						w.dbg_.tree5__(&a, idx, end, &w);
					}
					while idx < end {
						let i = &a[idx];
						match as_ref__!(i).kw__().id_ {
							keyword_::Id_::Undef => {
								let mut idx2 = 0;
								let cs2 = cs__(i);
								while let Some((idx4, len)) = f__(&cs2, idx2, &mut i_argname, false) {
									dunhao3(idx2, idx4, cs2.len(), &i, &mut a2);
									idx2 = idx4 + len;
								}
								if idx2 > 0 {
									dunhao3(idx2, cs2.len(), cs2.len(), &i, &mut a2);
								} else {
									a2.push(i.clone());
								}
							}
							keyword_::Id_::Dunhao => {
								if i_argname < argnames.len() {
									let argname = &argnames[i_argname];
									if argname.is_ge_ {
										paichu(idx2);
										#[cfg(debug_assertions)]
										if db_c__!("-sparg-", env) {
											lc6__!("(b,)");
										}
										continue 'l1
									}
								} else {
									break
								}
								i_argname += 1;
								a2.push(i.clone());
							}
							keyword_::Id_::Jvhao => break,
							_ => a2.push(i.clone())
						}
						idx += 1
					}
					if i_argname < argnames.len() {
						paichu(idx2);
						#[cfg(debug_assertions)]
						if db_c__!("-sparg-", env) {
							lc6__!("(bl)");
						}
						continue 'l1
					}
					end2a = idx;
				}
			}
			paichu_def.clear();
			*has = true;
			*idx2 += len;
			while idx2_save < idx3 {
				as_mut_ref__!(env.ret).add__(cs[idx2_save]);
				idx2_save += 1
			}
			match &def.val_ {
				def_::Val_::Si(s) => as_mut_ref__!(env.ret).add__(s),
				_ => {
					let q2 = qv_::Qv_::new5(def.name__(), def.names__(), Some(env.q.clone()));
					let q2 = t__(q2);
					if def.argc__() > 0 {
						let args = &as_mut_ref__!(q2).args_;
						if end2a == 0 {
							z4__(a, cs, idx2, end2, has, idx, end, &Env_::new6(args.clone(), env))?;
						} else {
							#[cfg(debug_assertions)]
							if db_c__!("-sparg-", env) {
								let w = as_ref__!(env.w);
								w.dbg_.tree6__(&a2, &w);
							}
							z2__(&a2, &mut 0, a2.len(), &Env_::new6(args.clone(), env))?;
							*idx2 = end2;
							*idx = end2a;
						}
						z2__(a, idx, end,
							&Env_::new7(Opt_ {jvhao_:true, ..env.gd}, args.clone(), env))?;
						if *idx >= end {
							*idx -= 1
						}
					}

					match &def.val_ {
						def_::Val_::S(src) => {
							let mut codes = None;
							as_mut_ref__!(env.w).codes_cache__(&src, |_| {}, |i| codes = Some(i.unwrap().clone()))?;
							if def.backarg_ > 0 {
								let q2 = as_ref__!(q2);
								def_::backarg_::backarg__(&mut as_mut_ref__!(env.ret), def.backarg_, &as_ref__!(env.w),
									&mut as_mut_ref__!(q2.args_.clone()))?;
							}
							let env2 = Env_::new11(Opt_ {jvhao_:false, ..env.gd}, Some(def_q), q2, env);
							eval_::return__(hello__(&codes.unwrap(), &env2), &env2)?;
						}
						def_::Val_::F(f) => {
							as_mut_ref__!(q2).objs_ = Some(def.objs_.clone());
							let env2 = Env_::new5(Opt_ {jvhao_:false, ..env.gd}, q2, env);
							f(&env2)?;
						}
						_ => {}
					}
				}
			}
		} else {
			if *has {
				buf.push(cs[*idx2]);
			}
			*idx2 += 1
		}
	}
	if *has {
		if !buf.is_empty() {
			as_mut_ref__!(env.ret).add__(buf);
		}
		*idx += 1;
	}
	ok__()
}
