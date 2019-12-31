use super::*;
use super::super::*;
use super::super::u2_::*;
use std::fs::File;
use std::io::{Read, Error};
//use std::path::PathBuf;

pub fn src__(s:&str, src2:&mut String, q:&mut Qv_, w:&mut World_) -> Result2_ {
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
							w.clpars__(&mut str_::split__(&src2[2..i]).into_iter(), true, false, &mut cfg, q)?;
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

pub fn hello__(src:&str, gd:code_::Opt_, q2:qv_::T_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
	let mut codes:codes_cache_::OI_<code_::List_> = None;
	w.codes_cache__(src, |i| codes = Some(i.unwrap().clone()))?;
	let codes = codes.unwrap();
	result2_::item__(code_::hello__(&codes, gd, q2, w, ret), |ret| {
		if let Err((jump_::RETURN_, s, _)) = &ret {
			if w.dbg_.lc_ {
				lc3__!("({}={})", s, q.borrow().name_);
			}
			if s.is_empty() || s == &q.borrow().name_ {
				return ok__()
			}
		}
		ret
	})
}
