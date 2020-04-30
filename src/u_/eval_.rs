use super::{*, super::{u2_::*, {lc3__, lc_kw__, p__, as_ref__}}};
use std::{fs::File, io::{Read, Error}/*, path::PathBuf*/};

pub fn src__(s:&str, src2:&mut String, q:&mut Qv_, w:&World_, dbg:&mut Dbg_) -> Result2_ {
	let e__ = |e:Error| {
		result2_::err__(["\"", s, "\" ", &e.to_string()].concat())
	};
	match File::open(s) {
		Ok(mut f) => {
			//File { fd: 3, path: "", read: true, write: false }
			/*let pb = PathBuf::from(s);
			println!("{:?} {:?}",pb.file_name(),pb.parent().unwrap());*/
			match f.read_to_string(src2) {
				Ok(_) => {
					if src2.starts_with("#!") {
						if let Some(i) = src2.find('\n') {
							let mut cfg = Cfg_::new();
							w.clpars__(&mut str_::split__(&src2[2..i]).into_iter(), true, false, false,
								&mut cfg, dbg, q)?;
							*src2 = src2[i+1..].to_string();
						}
					}
					//q.src2_ = 
				},
				Err(e) => {
					return e__(e)
				}
			}
		}
		Err(e) => {
			return e__(e)
		}
	}
	ok__()
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
