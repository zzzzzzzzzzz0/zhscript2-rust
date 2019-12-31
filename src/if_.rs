use super::u_::*;

pub const OK_:i32 = 3000;

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
	fn cmp__(&self, cmp:&mut Cmp_, b:&mut bool, w:&mut World_) {
		if cmp.op_.id_ == self.undef_.id_ {
			if w.dbg_.if_ {
				lc3__!("\n({}{:?})", t_::b__(*b), cmp.val_);
			}
			for i in &cmp.val_ {
				match i.as_str() {
					"" | "0" | "false" | "NULL" => {}
					_ => {
						*b = true;
						break
					}
				}
			}
			return
		}
		let mut i = 0;
		let mut i2 = 0;
		let no = core::usize::MAX;
		let end = if cmp.left_val_.is_empty() {no} else {cmp.left_val_.len() - 1};
		let end2 = if cmp.val_.is_empty() {no} else {cmp.val_.len() - 1};
		loop {
			let lv = if end == no {""} else {&cmp.left_val_[i]};
			let rv = if end2 == no {""} else {&cmp.val_[i2]};
			if w.dbg_.if_ {
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
			if cmp.not_ {
				*b = !*b
			}
			if w.dbg_.if_ {
				lc3__!("{}{})", t_::b__(*b), t_::b__(cmp.not_));
			}
			if *b {
				return
			}
			if (end == no || i == end) && (end2 == no || i2 == end2) {
				break
			}
			if i < end {
				i += 1
			}
			if i2 < end2 {
				i2 += 1
			}
		}
	}
	
	fn b__(&self, b:&mut bool, q:qv_::T_, w:&mut World_) -> Result2_ {
		self.b2__(t_::o__(&self.a_), b, q, w)
	}
	fn b2__(&self, codes:&code_::List_, b:&mut bool, q:qv_::T_, w:&mut World_) -> Result2_ {
		let mut idx = 0;
		let mut ret2 = result_::List_::new();
		let mut cmp = Cmp_{left_val_:vec![], val_:vec![], op_:self.undef_.clone(),
			not_:false, from_:0};
		while idx < codes.len() {
			let i = &codes[idx];
			let kw = &i.kw__();
			let id = &kw.id_;
			if *id == keyword_::Id_::BeginBlock {
				self.b2__(i.a__().unwrap(), b, q.clone(), w)?;
			} else if let Some(kw2) = self.kws_.iter().find(|kw2| kw2.id_ == *id) {
				if *id == self.or_.id_ || *id == self.and_.id_ {
					cmp.mv__(codes, idx, q.clone(), w, &mut ret2)?;
					self.cmp__(&mut cmp, b, w);
					if w.dbg_.if_ {
						lc3__!("\n({}", t_::b__(*b));
						lc5__!("{}", kw.s_);
						lc3__!(")");
					}
					if *id == self.or_.id_ {
						if *b {
							return result2_::n__(OK_)
						}
					} else if !*b {
							return result2_::n__(OK_)
					}
				} else if *id == self.not_.id_ {
					cmp.not_ = !cmp.not_;
				} else {
					cmp.op_ = kw2.clone();
					cmp.mv__(codes, idx, q.clone(), w, &mut ret2)?;
					idx = cmp.from_;
					continue
				}
			}
			idx += 1;
		}
		cmp.mv__(codes, idx, q.clone(), w, &mut ret2)?;
		self.cmp__(&mut cmp, b, w);
		result2_::n__(OK_)
	}
}

struct Cmp_ {
	left_val_:Vec<String>,
	not_:bool,
	op_:keyword_::RI_,
	val_:Vec<String>,
	from_:usize,
}

impl Cmp_ {
	fn mv__(&mut self, codes:&code_::List_, idx:usize,
			q:qv_::T_, w:&mut World_, ret2:&mut result_::List_) -> Result2_ {
		codes.hello2__(&mut self.from_, idx, code_::Opt_::new(), q, w, ret2)?;
		
		//self.left_val_ = self.val_;
		self.left_val_.clear();
		self.left_val_.append(&mut self.val_);

		self.val_ = ret2.to_vec2__(false);
		ret2.clear();
		self.from_ += 1;
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
	fn a__(&self) -> code_::ORL_ {t_::some__(&&self.a_)}
	fn a2__(&self) -> code_::ORL_ {t_::some__(&&self.then_)}
	fn a3__(&self) -> code_::ORL_ {t_::some__(&self.else_)}

	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		let mut b = false;
		if w.dbg_.if_ {
			println!();
		}
		result2_::item__(self.b__(&mut b, q.clone(), w), |i| {
			if let Err((OK_, _, _)) = i {
				ok__()
			} else {
				i
			}
		})?;
		if b {
			if w.dbg_.lc_ {
				w.dbg_.lc_kw__(t_::or__(&self.super_.kw2__()));
			}
			t_::o__(&self.then_).hello__(gd, q.clone(), w, ret)
		} else {
			if w.dbg_.lc_ {
				w.dbg_.lc_kw__(t_::or__(&self.super_.kw3__()));
			}
			t_::o__(&self.else_).hello__(gd, q.clone(), w, ret)
		}
	}
}
