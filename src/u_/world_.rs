use super::{*, super::{u2_::*, as_mut_ref__, as_ref__}};

pub const HELP_: &str = "-zhscript-help";

type Pars_ = pars_::Pars_;

pub type T_ = super::T_<World_>;

#[derive(Clone)]
pub struct World_ {
	pub cfg_:Cfg_,
	pub dbg_:Dbg_,
	pub pars_:Pars_,
	
	pub top_q_:qv_::T_,
	pub kws_:keyword_::List_,
	pub mods_:Vec<qv_::T_>,

	codes_cache_:codes_cache_::List_<code_::List_>,
	pub codes_cache2_:codes_cache_::List_<expl_::List_>,
}

impl World_ {
	pub fn new() -> Self {
		let kws_ = keyword_::List_::new();
		
		let mut q = Qv_::new("顶");
		q.simp_def__("换行", "\n").unwrap();
		q.simp_def__("回车", "\r").unwrap();
		q.simp_def__("制表符", "\t").unwrap();
		q.simp_def__("ESC", "\x1b").unwrap();
		
		Self {pars_:Default::default(), top_q_:t__(q), kws_, mods_:vec![],
			cfg_:Default::default(), dbg_:Default::default(),
			codes_cache_:Default::default(), codes_cache2_:Default::default()}
	}

	pub fn codes_cache__(&mut self, src:&str, fit:impl Fn(&mut IsText_),
			mut f:impl FnMut(codes_cache_::ORI_<code_::List_>)) -> Result2_ {
		if self.codes_cache_.get__(src).is_none() {
			let mut codes = code_::List_::new();
			self.pars__(src, fit, &mut codes)?;
			self.codes_cache_.set__(src, codes);
		}
		f(self.codes_cache_.get__(src));
		ok__()
	}
	
	pub fn pars__(&mut self, src:&str, fit:impl Fn(&mut IsText_), codes:&mut code_::List_) -> Result2_ {
		self.pars_.hello__(src, fit, codes, self, &self.dbg_)?;
		if self.dbg_.tree_ {
			self.dbg_.tree__(&codes, self)
		}
		ok__()
	}
	
	pub fn text__(&self, s:&str) -> String {
		[&self.kws_.begin_text_.s_, s, &self.kws_.end_text_.s_].concat()
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
	
	pub fn ret__(&self, ret:Result2_) -> i32 {
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
	pub fn ret3__(&self, mut ret:Result2_, a2:&[u8], from:usize, to:usize) -> Result2_ {
		if let Err((_, _, s)) = &mut ret {
			s.push('\n');
			s.push_str(&self.kws_.begin_text_.s_);
			s.push_str(&String::from_utf8(a2[from..to].to_vec()).unwrap());
			s.push_str(&self.kws_.end_text_.s_);
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

	pub fn hello3__(mut self, a:&mut clpars_::A_, has_shl:bool, has_src:bool, other_z:bool,
			q:&mut Qv_, ret:super::T_<result_::List_>) -> Result2_ {
		let mut src = String::new();
		clpars__(&mut self, a, has_shl, has_src, other_z, q)?;
		if has_shl {
			let mut top_q = as_mut_ref__!(self.top_q_);
			top_q.val__("外壳", &self.cfg_.shl_);
			top_q.val__("窗口", "linux");
		}
		if self.dbg_.arg_ || self.dbg_.path_ {
			println!();
			self.dbg_.arg2__(&self.cfg_.shl_);
		}
		if !has_src {
			return ok__()
		}
		let w = t__(self);
		let q2 = t__(q.clone());
		if as_ref__!(w).cfg_.src_is_file_ {
			eval_::ok_src__(&q.src_, q2.clone(), w.clone());
			eval_::src__(&mut src, q2.clone(), w.clone())?;
		} else {
			src.push_str(&q.src_)
		};
		eval_::hello__(&src, &code_::Env_::new(q2, w, ret))
	}
	/*pub*/ fn hello2__(self, a:&mut clpars_::A_, has_shl:bool, has_src:bool, other_z:bool, q:&mut Qv_) -> Result2_ {
		self.hello3__(a, has_shl, has_src, other_z, q, t__(result_::List_::new()))
	}
	pub fn hello__(self, a:&mut clpars_::A_) -> i32 {
		let mut q = Qv_::new2(Some(self.clone().top_q_));
		let ret = self.clone().hello2__(a, true, true, true, &mut q);
		self.ret__(ret)
	}
}

pub fn clpars__(self1:&mut World_, a:&mut clpars_::A_, has_shl:bool, has_src:bool, other_z:bool,
		q:&mut Qv_) -> Result2_ {
	let cfg = &mut self1.cfg_;
	let dbg = &mut self1.dbg_;
	let kws = &self1.kws_;
	if has_shl {
		let shl = a.next().unwrap();
		if shl.starts_with('/') || cfg.shl_.is_empty() {
			cfg.shl_ = shl;
		}
	}
	let mut has_src = has_src;
	let mut a2 = vec![];
	{
		let shebang_flag = ["--", &kws.jvhao_.s_].concat();
		let mut is1 = false;
		for s in a {
			let add__ = |a2:&mut Vec<String>| {
				if s.len() <= shebang_flag.len() {return}
				let s3 = &s[0..s.len() - 1 - shebang_flag.len()];
				for s2 in str_::split__(s3) {
					a2.push(s2)
				}
			};
			let is2 = s.ends_with(&shebang_flag);
			if has_src {
				if is2 {
					is1 = true;
					if other_z {
						a2.push("".to_string())
					}
					add__(&mut a2);
					continue
				}
				if is1 {
					is1 = false;
					has_src = false;
					q.src_ = s.to_string();
					continue
				}
			} else if is2 {
				add__(&mut a2);
				continue
			}
			a2.push(s)
		}
	}
	let mut args = as_mut_ref__!(q.args_);
	let src = &mut q.src_;
	let mut clpars;
	{
		let rem1 = "断点附出，计：";
		clpars = clpars_::List_::new2(vec![
			clpars_::Item_::new("-zhscript-src-is-code"),
			clpars_::Item_::new("-zhscript-d-tree"),
			clpars_::Item_::new("-zhscript-d-arg"),
			clpars_::Item_::new("-zhscript-d-path"),
			clpars_::Item_::new("-zhscript-d-lc"),
			clpars_::Item_::new("-zhscript-d-par-lc"),
			clpars_::Item_::new3t("-zhscript-d-parbp-", clpars_::Typ_::Starts, &["析", rem1, "-kw-、-2-、-l1-"].concat()),
			clpars_::Item_::new3t("-zhscript-d-bp-", clpars_::Typ_::Starts, &[rem1, "-r_v-"].concat()),
			clpars_::Item_::new("-zhscript-d-expl"),
			clpars_::Item_::new("-zhscript-d-if"),
			clpars_::Item_::new(HELP_),
			clpars_::Item_::new3t("-zhscript-", clpars_::Typ_::Starts, "以此为头之其他将忽略"),
		]);
	}
	if other_z {
		clpars.a_.push(clpars_::Item_::new0z());
	} else {
		clpars.a_.push(clpars_::Item_::new0());
	}
	
	let ret = clpars.for__(&mut a2.into_iter(), |tag, argv, item, i3| {
		if let clpars_::Typ_::Starts = item.typ_ {
			let bp__ = |s:&mut String| {
				s.push_str(&tag[item.tag_.len() - 1..])
			};
			match item.tag_.as_str() {
				"-zhscript-d-parbp-" => bp__(&mut dbg.parbp_),
				"-zhscript-d-bp-" => bp__(&mut dbg.bp_),
				_ => if i3 == 1 {eprintln!("忽略 {}", tag)}
			}
			return 0
		}
		match tag {
			"-zhscript-src-is-code" => cfg.src_is_file_ = false,
			"-zhscript-d-tree" =>
				dbg.tree_ = true,
			"-zhscript-d-arg" => dbg.arg_ = true,
			"-zhscript-d-path" => dbg.path_ = true,
			"-zhscript-d-lc" => dbg.lc_ = true,
			"-zhscript-d-par-lc" => dbg.par_lc_ = true,
			"-zhscript-d-expl" => dbg.expl_ = true,
			"-zhscript-d-if" => dbg.if_ = true,
			"-zhscript-help" => return 251,
			_ => {
				if has_src {
					*src = tag.to_string();
				}
				let mut add__ = |i:&str| {
					if !args.is_empty() {
						//self.dunhao__(args);
						args.add2__(kws.dunhao_.clone());
					}
					args.add__(i);
				};
				if other_z {
					for i in argv.iter() {
						add__(i)
					}
				} else if !has_src {
					add__(tag)
				}
				if has_src {
					has_src = false;
				}
			}
		}
		0
	}, |_| 0);
	match ret {
		0 => ok__(),
		_ => result2_::n__(ret),
	}
}
