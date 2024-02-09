use super::u_::*;
#[cfg(debug_assertions)]
use super::db_c__;

pub const OK_:i32 = 3000;
pub const CT_:i32 = 3001;

pub struct Item_ {
	pub super_:code_::Item1_,
	dengyu_:keyword_::RI_,
	xiaoyudengyu_:keyword_::RI_,
	xiaoyu_:keyword_::RI_,
	dayudengyu_:keyword_::RI_,
	dayu_:keyword_::RI_,
	not_:keyword_::RI_,
	and_:keyword_::RI_,
	or_:keyword_::RI_,
	pub kws_:Vec<keyword_::RI_>,
	undef_:keyword_::RI_,
	pub a_:code_::OL_,
	pub then_:code_::OL_,
	pub else_:code_::OL_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:code_::Item1_::new3(&kws.if_, &kws.then_, &kws.else_),
			dengyu_:kws.dengyu_.clone(),
			xiaoyudengyu_:kws.xiaoyudengyu_.clone(),
			xiaoyu_:kws.xiaoyu_.clone(),
			dayudengyu_:kws.dayudengyu_.clone(),
			dayu_:kws.dayu_.clone(),
			not_:kws.not_.clone(),
			and_:kws.and_.clone(),
			or_:kws.or_.clone(),
			kws_:vec![
				kws.dengyu_.clone(),
				kws.xiaoyudengyu_.clone(),
				kws.xiaoyu_.clone(),
				kws.dayudengyu_.clone(),
				kws.dayu_.clone(),
				kws.not_.clone(),
				kws.and_.clone(),
				kws.or_.clone(),
			],
			undef_:kws.undef_.clone(),
			a_:None, then_:None, else_:None}
	}

	fn cmp2__(&self, lv:&str, rv:&str) -> i32 {
		if lv == rv {
			return 0
		}
		if let Some(n) = t_::s2n__::<f64>(lv) {
			if let Some(n2) = t_::s2n__(rv) {
				if n < n2 {
					return -1
				}
				if n > n2 {
					return 1
				}
				return 0
			}
		}
		let lv = lv.as_bytes();
		let rv = rv.as_bytes();
		let mut i = 0;
		let mut i2 = 0;
		loop {
			let b2 = i2 == rv.len();
			if i == lv.len() {
				if b2 {
					break
				}
				return -1
			}
			if b2 {
				return 1
			}
			let c = lv[i];
			let c2 = rv[i2];
			if c < c2 {
				return -1
			}
			if c > c2 {
				return 1
			}
			i += 1;
			i2 += 1;
		}
		0
	}
	fn cmp__(&self, cmp:&mut Cmp_, b:&mut bool, env:&code_::Env_) {
		let mut i = 0;
		let mut i2 = 0;
		const NO:usize = core::usize::MAX;
		let end = if cmp.left_val_.is_empty() {NO} else {cmp.left_val_.len() - 1};
		let end2 = if cmp.val_.is_empty() {NO} else {cmp.val_.len() - 1};
		loop {
			let rv = if end2 == NO {""} else {&cmp.val_[i2]};
			if cmp.op_.id_ == self.undef_.id_ {
				#[cfg(debug_assertions)]
				if db_c__!("-if-", env) {
					lc3__!("\n({}({})", t_::b__(*b), rv);
				}
				*b = t_::true__(rv)
			} else {
				let lv = if end == NO {""} else {&cmp.left_val_[i]};
				#[cfg(debug_assertions)]
				if db_c__!("-if-", env) {
					lc3__!("\n({}{}/{}-{}/{}", t_::b__(*b), i, end, i2, end2);
					lc3__!("({}", lv);
					lc5__!("{}", &cmp.op_.s_);
					lc3__!("{})", rv);
				}
				let ret = self.cmp2__(lv, rv);
				*b = if cmp.op_.id_ == self.dengyu_.id_ {
					ret == 0
				} else if cmp.op_.id_ == self.xiaoyudengyu_.id_ {
					ret <= 0
				} else if cmp.op_.id_ == self.xiaoyu_.id_ {
					ret < 0
				} else if cmp.op_.id_ == self.dayudengyu_.id_ {
					ret >= 0
				} else if cmp.op_.id_ == self.dayu_.id_ {
					ret > 0
				} else {
					false
				};
			}
			if cmp.not_ {
				*b = !*b
			}
			#[cfg(debug_assertions)]
			if db_c__!("-if-", env) {
				lc3__!("{}{})", t_::b__(*b), t_::b__(cmp.not_));
			}
			if *b {
				break
			}
			if (end == NO || i == end) && (end2 == NO || i2 == end2) {
				break
			}
			if i < end {
				i += 1
			}
			if i2 < end2 {
				i2 += 1
			}
		}
		cmp.not_ = false;
		cmp.op_ = self.undef_.clone();
	}
	
	fn b__(&self, b:&mut bool, env:&code_::Env_) -> Result2_ {
		self.b2__(t_::o__(&self.a_), b, env)
	}
	fn b2__(&self, codes:&code_::List_, b:&mut bool, env:&code_::Env_) -> Result2_ {
		let mut idx = 0;
		let mut cmp = Cmp_{left_val_:vec![], val_:vec![], op_:self.undef_.clone(),
			not_:false, from_:0, end_:0};
		let env2 = code_::Env_::new6(t__(result_::List_::new()), env);
		let mut kuo = false;
		let fcmp = |cmp:&mut Cmp_, b:&mut bool, kuo:&mut bool| {
			if *kuo {
				*kuo = false;
			} else {
				cmp.mv__(true, codes, &env2)?;
				self.cmp__(cmp, b, env);
			}
			ok__()
		};
		while idx < codes.len() {
			let i = &codes[idx];
			let r_i = as_ref__!(i);
			let kw = &r_i.kw__();
			let id = &kw.id_;
			if *id == keyword_::Id_::BeginBlock {
				kuo = true;
				let ret = self.b2__(as_ref__!(i).a__().unwrap(), b, &env2);
				if let Err((i, _, _, _)) = ret {
					if i == CT_ {} else {return ret}
				}
				cmp.from_ += 1;
			} else if let Some(kw2) = self.kws_.iter().find(|kw2| kw2.id_ == *id) {
				if *id == self.or_.id_ || *id == self.and_.id_ {
					cmp.end_ = idx;
					fcmp(&mut cmp, b, &mut kuo)?;
					#[cfg(debug_assertions)]
					if db_c__!("-if-", env) {
						lc3__!("\n({}", t_::b__(*b));
						lc5__!("{}", kw.s_);
						lc3__!(")");
					}
					if *id == self.or_.id_ && *b {
						return result2_::n__(OK_)
					}
					cmp.end_ = 0;
					if *id == self.and_.id_ && !*b {
						loop {
							idx += 1;
							if idx >= codes.len() {break}
							if as_ref__!(codes[idx]).kw__().id_ == keyword_::Id_::Or {break}
						}
						cmp.from_ = idx;
						kuo = true;
						continue
					}
					cmp.from_ = idx + 1;
				} else if *id == self.not_.id_ {
					cmp.not_ = !cmp.not_;
					if cmp.end_ == 0 {
						cmp.from_ = idx + 1;
					}
					else if cmp.end_ == usize::MAX {
						cmp.end_ = idx;
					}
				} else {
					cmp.op_ = kw2.clone();
					if cmp.end_ == usize::MAX {
						cmp.end_ = idx;
					}
					cmp.mv__(false, codes, &env2)?;
					cmp.from_ = idx + 1;
				}
			} else {
				cmp.end_ = usize::MAX;
			}
			idx += 1;
		}
		fcmp(&mut cmp, b, &mut kuo)?;
		result2_::n__(CT_)
	}
}

struct Cmp_ {
	left_val_:Vec<String>,
	not_:bool,
	op_:keyword_::RI_,
	val_:Vec<String>,
	from_:usize,
	end_:usize,
}

impl Cmp_ {
	fn mv__(&mut self, can:bool, codes:&code_::List_, env:&code_::Env_) -> Result2_ {
		#[cfg(debug_assertions)]
		if db_c__!("-if-", env) {
			let w = as_ref__!(env.w);
			w.dbg_.tree5__(&codes.a_, self.from_, self.end_, &w);
		}
		codes.hello2__(&mut self.from_, self.end_, &code_::Env_::new3(Default::default(), env))?;
		
		if can && !self.val_.is_empty() {
			self.left_val_.clear();
			self.left_val_.append(&mut self.val_);
		}

		self.val_ = as_mut_ref__!(env.ret).to_vec2__(false);
		as_mut_ref__!(env.ret).clear();
		ok__()
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn kw2__(&self) -> keyword_::ORI_ {self.super_.kw2__()}
	fn kw3__(&self) -> keyword_::ORI_ {self.super_.kw3__()}

	fn add__(&mut self, a:code_::List_) -> Result2_ {
		self.a_ = Some(a);
		ok__()
	}
	fn add2__(&mut self, a:code_::List_) -> Result2_ {
		self.then_ = Some(a);
		ok__()
	}
	fn add3__(&mut self, a:code_::List_) -> Result2_ {
		self.else_ = Some(a);
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}
	fn a2__(&self) -> code_::ORL_ {t_::some__(&self.then_)}
	fn a3__(&self) -> code_::ORL_ {t_::some__(&self.else_)}

	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		let mut b = false;
		result2_::item__(self.b__(&mut b, env), |i| {
			if let Err((i, _, _, _)) = i {
				if i == OK_ || i == CT_ {
					return ok__()
				}
			}
			i
		})?;
		#[cfg(debug_assertions)]
		if db_c__!("-if-", env) {
			lc3__!("(");
			lc5__!("{}", t_::b__(b));
			lc3__!(")");
		}
		if b {
			#[cfg(debug_assertions)]
			if as_ref__!(env.w).dbg_.lc_ {
				as_ref__!(env.w).dbg_.lc_kw__(t_::or__(&self.super_.kw2__()));
			}
			t_::o__(&self.then_).hello__(env)
		} else {
			#[cfg(debug_assertions)]
			if as_ref__!(env.w).dbg_.lc_ {
				as_ref__!(env.w).dbg_.lc_kw__(t_::or__(&self.super_.kw3__()));
			}
			t_::o__(&self.else_).hello__(env)
		}
	}
}
