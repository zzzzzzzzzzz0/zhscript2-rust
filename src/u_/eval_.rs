use super::{*, super::{u2_::*, {lc3__, lc_kw__, p__, as_ref__, as_mut_ref__}}};
use std::{fs::File, io::{Read}, path::{PathBuf}};

pub fn src__(src:&str, src2:&mut String, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_) -> Result2_ {
	let shl = wm.cfg_.shl_.to_string();
	let mut err = String::new();
	let mut ok = false;
	let mut op__ = |s:&str, y_e, ok:&mut bool| {
		if wm.dbg_.path_ {
			wm.dbg_.arg2__(s);
		}
		match File::open(s) {
			Ok(mut f) => {
				match f.read_to_string(src2) {
					Ok(_) => {
						let mut q = as_mut_ref__!(q);
						if src2.starts_with("#!") {
							if let Some(i) = src2.find('\n') {
								as_mut_ref__!(w).clpars__(&mut str_::split__(&src2[2..i]).into_iter(), true, false, false,
									&mut wm.cfg_, &mut wm.dbg_, &mut q)?;
								*src2 = src2[i+1..].to_string();
							}
						}
						q.src_ = s.to_string();
						q.src_is_file_ = true;
						*ok = true;
					},
					Err(e) => if y_e {
						err = e.to_string()
					}
				}
			}
			Err(e) => if y_e {
				err = e.to_string()
			}
		}
		ok__()
	};
	op__(src, true, &mut ok)?;
	if ok {
		return ok__()
	}
	if !src.starts_with('/') {
		let mut op2__ = |src3:&str, ok:&mut bool| {
			let mut p = PathBuf::from(src3);
			p.set_file_name(src);
			if let Some(s) = p.to_str() {
				op__(s, false, ok)?;
			}
			ok__()
		};
		op2__(&shl, &mut ok)?;
		if ok {
			return ok__()
		}
		if qv_::for__(q.clone(), w.clone(), |q, _, _| {
			let q2 = as_ref__!(q);
			if q2.src_is_file_ && op2__(&q2.src_, &mut ok).is_ok() && ok {
				Some(())
			} else {
				None
			}
		}).is_some() {return ok__()}
	}
	result2_::err__(["\"", src, "\" ", &err].concat())
}

pub fn hello__(src:&str, gd:code_::Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
	hello2__(src, |_| {}, gd, q, w, wm, ret)
}
pub fn hello2__(src:&str, fit:impl Fn(&mut IsText_),
		gd:code_::Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
	let mut codes = None;
	wm.codes_cache__(src, fit, w.clone(), |i| codes = Some(i.unwrap().clone()))?;
	let codes = codes.unwrap();
	result2_::item__(code_::hello__(&codes, gd, q.clone(), w, wm, ret), |ret| {
		if let Err((jump_::RETURN_, s, _)) = &ret {
			if wm.dbg_.lc_ {
				lc3__!("({}={:?})", s, as_ref__!(q).name_);
			}
			if s.is_empty() || as_ref__!(q).name_.contains(s) {
				return ok__()
			}
		}
		ret
	})
}
