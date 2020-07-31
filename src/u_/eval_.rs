use super::{*, super::{u2_::*, {lc3__, lc_kw__, p__, as_ref__, as_mut_ref__}}};
use std::{fs::File, io::{Read, Error as ioError}, path::{PathBuf}};

pub fn ok_src__(src:&str, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_) -> bool {
	let shl = wm.cfg_.shl_.to_string();
	let mut ok = false;
	let op__ = |s:&str, ok:&mut bool| {
		if wm.dbg_.path_ {
			wm.dbg_.arg2__(s);
		}
		if File::open(s).is_ok() {
			let mut q = as_mut_ref__!(q);
			q.src_ = s.to_string();
			q.src_is_file_ = true;
			*ok = true;
		}
	};
	op__(src, &mut ok);
	if ok {
		return ok
	}
	if !src.starts_with('/') {
		let op2__ = |src3:&str, ok:&mut bool| {
			let mut p = PathBuf::from(src3);
			p.set_file_name(src);
			if let Some(s) = p.to_str() {
				op__(s, ok);
			}
		};
		op2__(&shl, &mut ok);
		if ok {
			return ok
		}
		if qv_::for__(q.clone(), w, |q, _, _| {
			let q2 = as_ref__!(q);
			if q2.src_is_file_ {
				op2__(&q2.src_, &mut ok);
				if ok {
					return Some(())
				}
			}
			None
		}).is_some() {return ok}
	}
	ok
}

pub fn src__(src2:&mut String, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_) -> Result2_ {
	let src = as_ref__!(q).src_.clone();
	let err__ = |e:ioError| {
		result2_::err__(["\"", &src, "\" ", &e.to_string()].concat())
	};
	match File::open(&src) {
		Ok(mut f) => {
			match f.read_to_string(src2) {
				Ok(_) => {
					if src2.starts_with("#!") {
						if let Some(i) = src2.find('\n') {
							let mut q = as_mut_ref__!(q);
							as_mut_ref__!(w).clpars__(&mut str_::split__(&src2[2..i]).into_iter(), true, false, false,
								&mut wm.cfg_, &mut wm.dbg_, &mut q)?;
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

pub fn hello__(src:&str, env:&code_::Env_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
	hello2__(src, |_| {}, env, wm, ret)
}
pub fn hello2__(src:&str, fit:impl Fn(&mut IsText_),
		env:&code_::Env_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
	hello3__(src, fit, true, env, wm, ret)
}
pub fn hello3__(src:&str, fit:impl Fn(&mut IsText_), cache:bool,
		env:&code_::Env_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
	let mut codes = None;
	if cache {
		wm.codes_cache__(src, fit, env.w.clone(), |i| codes = Some(i.unwrap().clone()))?;
	} else {
		let mut codes2 = code_::List_::new();
		wm.pars__(src, fit, &mut codes2, env.w.clone())?;
		codes = Some(Rc_::new(codes2));
	}
	let codes = codes.unwrap();
	result2_::item__(code_::hello__(&codes, env, wm, ret), |ret| {
		if let Err((jump_::RETURN_, s, _)) = &ret {
			if wm.dbg_.lc_ {
				lc3__!("({}={:?})", s, as_ref__!(env.q).name_);
			}
			if s.is_empty() || as_ref__!(env.q).name_.contains(s) {
				return ok__()
			}
		}
		ret
	})
}
