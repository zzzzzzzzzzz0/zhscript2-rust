use super::{*, super::{u2_::*, as_mut_ref__, as_ref__}};

pub const HELP_: &str = "-zhscript-help";

pub const CR_:&str = "回车";
pub const LF_:&str = "换行";
pub const TAB_:&str = "制表符";
pub const ESC_:&str = "ESC";

pub const SHELL_: &str = "外壳";
pub const WIN_: &str = "窗口";

type Pars_ = pars_::Pars_;

pub type T_ = super::T_<World_>;

#[derive(Clone)]
pub struct World_ {
	pub cfg_:Cfg_,
	//#[cfg(debug_assertions)]
	pub dbg_:Dbg_,
	pub pars_:Pars_,
	pub shebang_flag_:String,
	
	pub top_q_:qv_::T_,
	pub kws_:keyword_::List_,
	pub mods_:Vec<qv_::T_>,
	pub datas_:Vec<qv_::T_>,
	path_:Vec<String>,

	codes_cache_:codes_cache_::List_<code_::List_>,
	//pub codes_cache2_:codes_cache_::List_<expl_::List_>,
}

impl World_ {
	pub fn new() -> Self {
		let kws_ = keyword_::List_::new();
		let shebang_flag_ = ["--", &kws_.jvhao_.s_].concat();
		
		let mut q = Qv_::new(qv_::rem4_::TOP_);
		q.simp_def__(LF_, "\n").unwrap();
		q.simp_def__(CR_, "\r").unwrap();
		q.simp_def__(TAB_, "\t").unwrap();
		q.simp_def__(ESC_, "\x1b").unwrap();
		
		Self {pars_:Default::default(), top_q_:t__(q), kws_, mods_:vec![], datas_:vec![], path_:vec![],
			shebang_flag_, cfg_:Default::default(),
			//#[cfg(debug_assertions)]
			dbg_:Default::default(),
			codes_cache_:Default::default(), /*codes_cache2_:Default::default()*/}
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

	pub fn path__(&self) -> &Vec<String> {&self.path_}
	pub fn path2__(&mut self, s:String) {
		let d__ = |s:&String| {
			if let Some(i) = s.rfind('/') {
				s[0..=i].to_string()
			} else {"".to_string()}
		};
		let d = d__(&s);
		if d.is_empty() {
			return;
		}
		for i in &self.path_ {
			if d__(i).eq(&d) {
				return;
			}
		}
		self.path_.push(s);
	}
	
	pub fn pars__(&mut self, src:&str, fit:impl Fn(&mut IsText_), codes:&mut code_::List_) -> Result2_ {
		self.pars_.hello__(src, fit, codes, self)?;
		if self.dbg_.tree_ {
			self.dbg_.tree6__(&codes, self)
		}
		ok__()
	}
	
	pub fn text__(&self, s:&str) -> String {
		[&self.kws_.begin_text_.s_, s, &self.kws_.end_text_.s_].concat()
	}
	pub fn rem1__(&self, ret:&mut String) {
		ret.insert_str(0, &self.kws_.begin_rem_.s_);
		ret.push_str(&self.kws_.end_rem_.s_);
	}
	pub fn rem__(&self, s:&str, ret:&mut String) {
		ret.push_str(&self.kws_.begin_rem2_.s_);
		ret.push_str(s);
		ret.push_str(&self.kws_.end_rem2_.s_);
	}
	pub fn no_rem2__(&self, s:&str) -> Result2_ {
		result2_::err__(["注解", &self.text__(&s), "不支持"].concat())
	}
	pub fn no_guandaojie__(&self, c:char) -> Result2_ {
		result2_::err__([&self.kws_.guandaojie_.s_, &self.text__(&c.to_string()), "不支持"].concat())
	}
	pub fn dunhao__(&self, ret:&mut result_::List_) {
		ret.add2__(self.kws_.dunhao_.clone())
	}
	
	pub fn ret__(&self, ret:Result2_) -> i32 {
		match ret {
			Ok(()) => 0,
			Err((i, i2, s, s2)) => {
				if i == jump_::QUIT_ && i2 != jump_::NO_ {
					eprint!("{}", s);
					return i2
				}
				result2_::eprtn__(i, i2, &s, &s2);
				result2_::exitcode__(i)
			}
		}
	}
	pub fn ret3__(&self, mut ret:Result2_, a2:&[u8], from:usize, to:usize) -> Result2_ {
		if let Err((_, _, _, s)) = &mut ret {
			s.push('\n');
			s.push_str(&self.kws_.begin_text_.s_);
			if let Ok(s2) = String::from_utf8(a2[from..to].to_vec()) {
				s.push_str(&s2);
			}
			s.push_str(&self.kws_.end_text_.s_);
		}
		ret
	}
	pub fn ret4__(&self, mut ret:Result2_, kw:&keyword_::Item_) -> Result2_ {
		if let Err((_, _, s, _)) = &mut ret {
			s.insert_str(0, &self.kws_.end_text_.s_);
			s.insert_str(0, &kw.s_);
			s.insert_str(0, &self.kws_.begin_text_.s_);
		}
		ret
	}

	pub fn hello3__(self, a:&mut clpars_::A_, has_shl:bool, has_src:bool, other_z:bool, q:&mut Qv_, ret:super::T_<result_::List_>) -> Result2_ {
		hello__(a, has_shl, has_src, other_z, q, t__(self), ret)
	}
	pub fn hello2__(self, a:&mut clpars_::A_, has_shl:bool, has_src:bool, other_z:bool, q:&mut Qv_) -> Result2_ {
		self.hello3__(a, has_shl, has_src, other_z, q, t__(result_::List_::new()))
	}
	pub fn hello__(self, a:&mut clpars_::A_) -> i32 {
		let mut q = Qv_::new2(Some(self.clone().top_q_));
		let ret = self.clone().hello2__(a, true, true, true, &mut q);
		self.ret__(ret)
	}
	#[allow(dead_code)]
	pub fn hello4__(&mut self, a:&mut clpars_::A_, ret:super::T_<result_::List_>) -> i32 {
		self.cfg_.src_is_file_ = false;
		let mut q = Qv_::new2(Some(self.top_q_.clone()));
		let ret = hello__(a, true, true, true, &mut q, t__(self.clone()), ret);
		self.ret__(ret)
	}
}

pub fn hello__(a:&mut clpars_::A_, has_shl:bool, has_src:bool, other_z:bool, q:&mut Qv_, w:T_, ret:super::T_<result_::List_>) -> Result2_ {
	let mut src = String::new();
	clpars__(a, has_shl, has_src, other_z, false, q, w.clone())?;
	let w1 = || as_ref__!(w);
	if has_shl {
		let top_q = &w1().top_q_;
		let mut top_q = as_mut_ref__!(top_q);
		top_q.val__(SHELL_, &w1().cfg_.shl_);
		top_q.val__(WIN_, "linux");
	}
	#[cfg(debug_assertions)]
	if w1().dbg_.arg_ || w1().dbg_.path_ {
		println!();
		w1().dbg_.arg2__(&w1().cfg_.shl_);
	}
	if !has_src {
		return ok__()
	}
	let q2 = t__(q.clone());
	if w1().cfg_.src_is_file_ {
		eval_::ok_src__(&q.src_, q2.clone(), w.clone());
		eval_::src__(&mut src, q2.clone(), w.clone())?;
	} else {
		src.push_str(&q.src_)
	};
	eval_::hello__(&src, &code_::Env_::new(q2, w.clone(), ret))
}

pub fn issue__() -> &'static str {
	concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"), "\n")
}

pub fn clpars__(a:&mut clpars_::A_, has_shl:bool, has_src:bool, other_z:bool, plus:bool, q:&mut Qv_, w:T_) -> Result2_ {
	let w1 = || as_ref__!(w);
	let w3 = || as_mut_ref__!(w);
	if has_shl {
		let shl = a.next().unwrap();
		if !w1().cfg_.shl_.starts_with('/') {
			w3().cfg_.shl_ = shl.clone();
		}
		w3().path2__(shl.clone());
	}
	let mut has_src = has_src;
	let mut a2 = vec![];
	{
		let mut is1 = false;
		let shebang_flag = &w1().shebang_flag_;
		for s in a {
			let add = |a2:&mut Vec<String>| {
				if !plus {return}
				if s.len() <= shebang_flag.len() {return}
				let s3 = &s[0..s.len() - 1 - shebang_flag.len()];
				for s2 in str_::split__(s3) {
					a2.push(s2)
				}
			};
			let is2 = s.ends_with(shebang_flag);
			if has_src {
				if is2 {
					is1 = true;
					if other_z {
						a2.push("".to_string())
					}
					add(&mut a2);
					continue
				}
				if is1 {
					is1 = false;
					has_src = false;
					q.src_ = s.to_string();
					continue
				}
			} else if is2 {
				add(&mut a2);
				continue
			}
			a2.push(s)
		}
	}
	let mut args = as_mut_ref__!(q.args_);
	let src = &mut q.src_;
	let mut clpars;
	{
		let mut v = vec![
			clpars_::Item_::new("-zhscript-src-is-code"),
			clpars_::Item_::new3("-zhscript-path", 1, ""),
			clpars_::Item_::new("-zhscript-jhdh"),
			clpars_::Item_::new("-zhscript-d-tree"),
			clpars_::Item_::new(HELP_),
			clpars_::Item_::new3t("-zhscript-", clpars_::Typ_::Starts, "以此为头之其他将忽略"),
		];
		#[cfg(debug_assertions)]
		{
			let rem1 = "断点附出，计：";
			let mut idx = 2;
			for i in vec![
				clpars_::Item_::new("-zhscript-d-arg"),
				clpars_::Item_::new("-zhscript-d-path"),
				clpars_::Item_::new("-zhscript-d-lc"),
				clpars_::Item_::new("-zhscript-d-lch"),
				clpars_::Item_::new("-zhscript-d-par-lc"),
				clpars_::Item_::new3t("-zhscript-d-parbp-", clpars_::Typ_::Starts,
					&["析", rem1, "-kw-、-2-、-l1-"].concat()),
				clpars_::Item_::new3t("-zhscript-d-bp-", clpars_::Typ_::Starts,
					&[rem1, "-ret-、-arg-、-tree-、-tree4-、-set-"].concat()),
				#[cfg(debug_assertions)]
				clpars_::Item_::new3t("-zhscript-d-s-", clpars_::Typ_::Starts, "-if- -expl- -sparg-"),
			] {
				v.insert(idx, i);
				idx += 1;
			}
		}
		clpars = clpars_::List_::new3(v, issue__());
	}
	if other_z {
		clpars.a_.push(clpars_::Item_::new0z());
	} else {
		clpars.a_.push(clpars_::Item_::new0());
	}
	
	clpars_ret__(clpars.for3__(&mut a2.into_iter(), |tag, argv, _, item, i3, _| {
		if let clpars_::Typ_::Starts = item.typ_ {
			#[cfg(debug_assertions)]
			{
				let bp = |s:&mut String| {
					s.push_str(&tag[item.tag_.len() - 1..])
				};
				let mut b = true;
				match item.tag_.as_str() {
					"-zhscript-d-parbp-" => bp(&mut w3().dbg_.parbp_),
					"-zhscript-d-bp-" => bp(&mut w3().dbg_.bp_),
					#[cfg(debug_assertions)]
					"-zhscript-d-s-" => bp(&mut w3().dbg_.s_),
					_ => b = false
				}
				if b {return 0}
			}
			if i3 == 1 {eprintln!("忽略 {}", tag)}
			return 0
		}
		#[cfg(debug_assertions)]
		{
			let mut b = true;
			match tag {
				"-zhscript-d-arg" => w3().dbg_.arg_ = true,
				"-zhscript-d-path" => w3().dbg_.path_ = true,
				"-zhscript-d-lc" => w3().dbg_.lc_ = true,
				"-zhscript-d-lch" => w3().dbg_.lch_ = true,
				"-zhscript-d-par-lc" => w3().dbg_.par_lc_ = true,
				_ => b = false
			}
			if b {return 0}
		}
		match tag {
			"-zhscript-src-is-code" => w3().cfg_.src_is_file_ = false,
			"-zhscript-path" => w3().path2__([argv[0].clone(), "/0".to_string()].concat()),
			"-zhscript-jhdh" => w3().cfg_.jvhao_dunhao_ = true,
			"-zhscript-d-tree" => w3().dbg_.tree_ = true,
			HELP_ => return clpars_::HELP_,
			_ => {
				let mut add = |i:&str| {
					if !args.is_empty() {
						args.add2__(w1().kws_.dunhao_.clone());
					}
					args.add__(i);
				};
				if has_src {
					if tag.starts_with('-') {
						add(tag)
					} else {
						*src = tag.to_string();
					}
				}
				if other_z {
					for i in argv.iter() {
						add(i)
					}
				} else if !has_src {
					add(tag)
				}
				if has_src {
					has_src = false;
				}
			}
		}
		0
	}, |_| 0), ok__(), w)
}
pub fn clpars_ret2__(ret3:i32, s:String, ret2:Result2_, w:T_) -> Result2_ {
	match ret3 {
		clpars_::ARG_NE_ | clpars_::TAG_NO_ => {
			let mut s2 = String::new();
			s2.push_str(&as_ref__!(w).text__(&s));
			s2.push_str(match ret3 {
				clpars_::ARG_NE_ => "参数不足",
				_ => "无效选项"
			});
			result2_::err2__(&s2)
		}
		clpars_::HELP_ => result2_::err__(s),
		_ => ret2
	}
}
pub fn clpars_ret__(ret3:clpars_::Result_, ret2:Result2_, w:T_) -> Result2_ {
	if let Err((ret3, s)) = ret3 {
		clpars_ret2__(ret3, s, ret2, w)
	} else {ret2}
}
