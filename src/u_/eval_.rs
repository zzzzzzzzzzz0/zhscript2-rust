use super::{*, super::{u2_::*, {as_ref__, as_mut_ref__}}};
#[cfg(debug_assertions)]
use super::super::{lc3__, lc_kw__, p__};
use std::{fs::File, io::{Read, Error as ioError}, path::{PathBuf}};

pub fn ok_src2__(src:&str, src2:bool, q:qv_::T_, w:world_::T_) -> bool {
	if src.is_empty() {
		false
	} else {
		let op__ = |s:&str| {
			#[cfg(debug_assertions)]
			if as_ref__!(w).dbg_.path_ {
				as_ref__!(w).dbg_.arg2__(s);
			}
			if File::open(s).is_ok() {
				let mut q = as_mut_ref__!(q);
				if src2 {
					q.src_ = s.to_string();
				}
				q.src_is_file_ = true;
				true
			} else {false}
		};
		if src.starts_with('/') {
		} else {
			let op2__ = |src3:&str| {
				let mut p = PathBuf::from(src3);
				p.set_file_name(src);
				if let Some(s) = p.to_str() {
					op__(s)
				} else {false}
			};
			for i in as_ref__!(w).path__() {
				if op2__(i) {
					return true
				}
			}
			if qv_::for3__(q.clone(), w.clone(), |q, _, _| {
				let q2 = as_ref__!(q);
				if q2.src_is_file_ {
					if op2__(&q2.src_) {
						return Some(())
					}
				}
				None
			}, true, true, false).is_some() {return true}
		}
		op__(src)
	}
}
pub fn ok_src__(src:&str, q:qv_::T_, w:world_::T_) -> bool {
	ok_src2__(src, true, q, w)
}

pub fn src2__(src:&String, src2:&mut String, q:qv_::T_, w:world_::T_) -> Result2_ {
	let err__ = |e:ioError| {
		result2_::err__(["\"", src, "\" ", &e.to_string()].concat())
	};
	match File::open(src) {
		Ok(mut f) => {
			match f.read_to_string(src2) {
				Ok(_) => {
					if src2.starts_with("#!") {
						if let Some(i) = src2.find('\n') {
							let mut q = as_mut_ref__!(q);
							world_::clpars__(&mut str_::split__(&src2[2..i]).into_iter(),
								true, false, false, true, &mut q, w)?;
							*src2 = src2[i+1..].to_string();
						}
					}
					ok__()
				},
				Err(e) => err__(e)
			}
		}
		Err(e) => err__(e)
	}
}
pub fn src__(src2:&mut String, q:qv_::T_, w:world_::T_) -> Result2_ {
	let src = as_ref__!(q).src_.clone();
	src2__(&src, src2, q, w)
}

pub fn return__(ret:Result2_, env:&code_::Env_) -> Result2_ {
	if let Err((jump_::RETURN_, _, s, _)) = &ret {
		#[cfg(debug_assertions)]
		if as_ref__!(env.w).dbg_.lc_ {
			lc3__!("({}={:?})", s, as_ref__!(env.q).name_);
		}
		if s.is_empty() || as_ref__!(env.q).name_.contains(s) {
			return ok__()
		}
	}
	ret
}

pub fn codes__(src:&str, fit:impl Fn(&mut IsText_), cache:bool, env:&code_::Env_) -> Result<Option<Rc_<code_::List_>>, Result2_> {
	let mut codes = None;
	if cache {
		let ret = as_mut_ref__!(env.w).codes_cache__(src, fit, |i| codes = Some(i.unwrap().clone()));
		if ret.is_err() {
			return Err(ret)
		}
	} else {
		let mut codes2 = code_::List_::new();
		let ret = as_mut_ref__!(env.w).pars__(src, fit, &mut codes2);
		if ret.is_err() {
			return Err(ret)
		}
		codes = Some(Rc_::new(codes2));
	}
	Ok(codes)
}

pub fn hello__(src:&str, env:&code_::Env_) -> Result2_ {
	hello2__(src, |_| {}, env)
}
pub fn hello2__(src:&str, fit:impl Fn(&mut IsText_), env:&code_::Env_) -> Result2_ {
	hello3__(src, fit, true, env)
}
pub fn hello3__(src:&str, fit:impl Fn(&mut IsText_), cache:bool, env:&code_::Env_) -> Result2_ {
	let codes = match codes__(src, fit, cache, env) {
		Ok(codes) => codes.unwrap(),
		Err(ret) => return ret,
	};
	hello4__(codes, env)
}
pub fn hello4__(codes:Rc_<code_::List_>, env:&code_::Env_) -> Result2_ {
	result2_::item__(code_::hello__(&codes, &code_::Env_::new3(
		code_::Opt_ {guandao_du_:env.gd.guandao_du_, guandao_jie_:env.gd.guandao_jie_, ..Default::default()},
		env)), |ret| {
		return__(ret, env)
	})
}
