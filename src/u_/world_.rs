use super::super::u2_::*;
use super::*;

type Dbg_ = dbg_::Dbg_;
type Pars_ = pars_::Pars_;

pub struct World_ {
	pub dbg_:Dbg_,
	pars_:Pars_,
	
	pub top_q_:qv_::T_,
	pub kws_:keyword_::List_,
	pub mods_:Vec<qv_::T_>,

	codes_cache_:codes_cache_::List_<code_::List_>,
	pub codes_cache2_:codes_cache_::List_<expl_::List_>,
}

impl World_ {
	pub fn new() -> Self {
		let kws = keyword_::List_::new();
		
		let mut q = Qv_::new();
		q.simp_def__("换行", "\n");
		q.simp_def__("回车", "\r");
		q.simp_def__("制表符", "\t");
		q.simp_def__("ESC", "\x1b");
		
		Self {dbg_:Dbg_::new(), pars_:Pars_{}, top_q_:qv_::t__(q), kws_:kws, mods_:vec![],
			codes_cache_:codes_cache_::List_::new(), codes_cache2_:codes_cache_::List_::new()}
	}
	
	pub fn text__(&self, s:&str) -> String {
		[&self.kws_.begin_text_.s_, s, &self.kws_.end_text_.s_].concat()
	}
	pub fn text2__(&self, s:&str, ret:&mut String) {
		ret.push_str(&self.kws_.begin_text_.s_);
		ret.push_str(s);
		ret.push_str(&self.kws_.end_text_.s_);
	}
	pub fn rem__(&self, s:&str, ret:&mut String) {
		ret.push_str(&self.kws_.begin_rem2_.s_);
		ret.push_str(s);
		ret.push_str(&self.kws_.end_rem2_.s_);
	}
	pub fn no_rem2__(&self, s:&str) -> Result2_ {
		result2_::err__(["注解", &self.text__(&s), "不支持"].concat())
	}
	pub fn dunhao__(&self, ret:&mut result_::List_) {
		ret.add2__(self.kws_.dunhao_.clone())
	}
		
	pub fn clpars__(&mut self, a:&mut clpars_::A_, has_shl:bool, has_src:bool, cfg:&mut Cfg_, q:&mut Qv_) -> Result2_ {
		if has_shl {
			cfg.shl_ = a.next().unwrap().to_string();
		}
		let mut has_src = has_src;
		let args = &mut q.args_;
		let src = &mut q.src_;
		let dbg = &mut self.dbg_;
		let mut clpars = clpars_::List_::new(vec![
			clpars_::Item_::new("-zhscript-src-is-code"),
			clpars_::Item_::new("-zhscript-d-tree"),
			clpars_::Item_::new("-zhscript-d-arg"),
			clpars_::Item_::new("-zhscript-d-lc"),
			clpars_::Item_::new("-zhscript-d-par-lc"),
			clpars_::Item_::new("-zhscript-d-expl"),
			clpars_::Item_::new("-zhscript-d-if"),
			clpars_::Item_::new("-zhscript-help"),
			clpars_::Item_::new3t("-zhscript-", clpars_::Typ_::Starts, "以此为头之其他将忽略"),
		]);
		if has_src {
			clpars.a_.push(clpars_::Item_::new0z());
		} else {
			clpars.a_.push(clpars_::Item_::new0());
		}
		
		let kws = &self.kws_;
		let ret = clpars.for__(a, |tag, argv, item, i3| {
			if clpars_::Typ_::Starts == item.typ_ && i3 == 1 {
				eprintln!("忽略 {}", tag);
				return 0
			}
			match tag {
				"-zhscript-src-is-code" => cfg.src_is_file_ = false,
				"-zhscript-d-tree" => dbg.tree_ = true,
				"-zhscript-d-arg" => dbg.arg_ = true,
				"-zhscript-d-lc" => dbg.lc_ = true,
				"-zhscript-d-par-lc" => dbg.par_lc_ = true,
				"-zhscript-d-expl" => dbg.expl_ = true,
				"-zhscript-d-if" => dbg.if_ = true,
				"-zhscript-help" => {
					println!("{}", clpars.help__());
					return 255
				}
				_ => {
					if has_src {
						has_src = false;
						*src = tag.to_string();
						for i in argv.iter() {
							if !args.is_empty() {
								args.add2__(kws.dunhao_.clone())
							}
							args.add__(i);
						}
						return 0
					}
					if !args.is_empty() {
						//self.dunhao__(args)
						args.add2__(kws.dunhao_.clone())
					}
					args.add__(tag);
				}
			}
			0
		}, |_s| 0);
		match ret {
			0 => ok__(),
			_ => result2_::n__(ret),
		}
	}
	
	fn pars__(&self, src:&str, codes:&mut code_::List_) -> Result2_ {
		self.pars_.hello__(src, codes, self)?;
		if self.dbg_.tree_ {
			self.dbg_.tree__(&codes, self)
		}
		ok__()
	}
	pub fn codes_cache__(&mut self, src:&str, mut f:impl FnMut(codes_cache_::ORI_<code_::List_>)) -> Result2_ {
		if self.codes_cache_.get__(src).is_none() {
			let mut codes = code_::List_::new();
			self.pars__(src, &mut codes)?;
			self.codes_cache_.set__(src, codes);
		}
		f(self.codes_cache_.get__(src));
		ok__()
	}
	
	fn ret__(&self, ret:Result2_) -> i32 {
		match ret {
			Ok(()) => 0,
			Err((i, s, s2)) => {
				if i == jump_::QUIT_ {
					if s.is_empty() {
						return 0
					}
					if let Some(i) = t_::s2n__(&s) {
						return i
					}
				}
				eprintln!("{}{}", s, s2);
				result2_::exitcode__(i)
			}
		}
	}
	/*pub fn ret2__(&self, ret:Result2_) -> bool {
		self.ret__(ret) != 0
	}*/
	pub fn ret3__(&self, mut ret:Result2_, a2:&[u8], from:usize, to:usize) -> Result2_ {
		if let Err((_, _, s)) = &mut ret {
			s.push('\n');
			self.text2__(&String::from_utf8(a2[from..to].to_vec()).unwrap(), s);
		}
		ret
	}
	pub fn ret4__(&self, mut ret:Result2_, kw:&keyword_::Item_) -> Result2_ {
		if let Err((_, s, _)) = &mut ret {
			s.insert_str(0, &self.kws_.end_text_.s_);
			s.insert_str(0, &kw.s_);
			s.insert_str(0, &self.kws_.begin_text_.s_);
		}
		ret
	}
	
	pub fn hello__(&mut self, a:&mut clpars_::A_, has_shl: bool) -> i32 {
		let mut q = Qv_::new2(Some(self.top_q_.clone()));
		let mut cfg = Cfg_::new();
		let ret = self.clpars__(a, has_shl, true, &mut cfg, &mut q);
		let ret = self.ret__(ret);
		if ret != 0 {
			return ret
		}
		self.top_q_.borrow_mut().val__("外壳", &cfg.shl_);
		self.top_q_.borrow_mut().val__("窗口", "linux");
		if self.dbg_.arg_ {
			println!();
			self.dbg_.arg2__(&cfg.shl_);
			self.dbg_.arg2__(&q.src_);
		}
		let mut src = String::new();
		if cfg.src_is_file_ {
			let src2 = q.src_.to_string();
			let ret = eval_::src__(&src2, &mut src, &mut q, self);
			let ret = self.ret__(ret);
			if ret != 0 {
				return ret
			}
		} else {
			src.push_str(&q.src_)
		};
		let ret = eval_::hello__(&src, code_::Opt_::new(), qv_::t__(q), qv_::t__(Qv_::new()), self, &mut result_::List_::new());
		self.ret__(ret)
	}
}
