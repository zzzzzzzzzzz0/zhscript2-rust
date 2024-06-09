use super::{*, super::{as_ref__}};
#[cfg(debug_assertions)]
use super::super::{lc3__, lc_kw__, p__};

pub const     BREAK_:i32 = 11032;
pub const  CONTINUE_:i32 = 11033;
pub const    BREAK2_:i32 = 11035;
pub const CONTINUE2_:i32 = 11036;
pub const    RETURN_:i32 = 11062;
pub const      QUIT_:i32 = 11061;

pub const NO_:i32 = 1000;

#[allow(dead_code)]
pub fn for_err__(ret2:&Result2_) -> (bool, bool) {
	if ret2.is_err() {
		if let Err((i, _, ref s, _)) = ret2 {
			if *i == BREAK_ && s.is_empty() {
				return (false, true);
			}
			if *i == CONTINUE_ && s.is_empty() {
				return (false, false);
			}
		}
		return (true, false);
	}
	(false, false)
}
#[allow(dead_code)]
pub fn for_err2__(ret2:Result2_, only_b:&mut bool) -> Result2_ {
	let (is_err, only_b1) = for_err__(&ret2);
	*only_b = only_b1;
	if is_err {ret2} else {ok__()}
}

pub fn return__(ret:Result2_, env:&code_::Env_) -> Result2_ {
	if let Err((RETURN_, i2, _, _)) = &ret {
		let q = as_ref__!(env.q);
		#[cfg(debug_assertions)]
		if as_ref__!(env.w).dbg_.lc_ 
		{
			lc3__!("(q.i_{}={})", q.i_, i2);
		}
		if q.i_ <= *i2 {
			return ok__()
		}
	}
	ret
}

pub trait Item_ : code_::Item_ {
	fn i__(&self) -> i32;
	fn b__(&self) -> bool {true}
	
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		let ret2 = t__(result_::List_::new());
		t_::o__(&self.a__()).hello__(&code_::Env_::new6(ret2.clone(), env))?;
		let v = as_ref__!(ret2).to_vec__();
		let mut s = String::new();
		let mut i = NO_;
		let mut q_i = -1;
		let mut up = 0;
		if !v.is_empty() {
			for idx in 0..v.len() {
				s = v[idx].to_string();
				match self.i__() {
					RETURN_ =>  {
						match s.as_str() {
							qv_::rem4_::UP_ => up += 1,
							_ =>
							if q_i < 0 {
								if let Some(i2) = qv_::for__(env.q.clone(), env.w.clone(), |q, _, _| {
									let q = as_ref__!(q);
									#[cfg(debug_assertions)]
									if as_ref__!(env.w).dbg_.lc_ {
										lc3__!("({}={:?})", s, q.name_);
									}
									if q.name_.contains(&s) {
										Some(q.i_)
									} else {None}
								}) {
									q_i = i2;
								}
							}
						}
					}
					BREAK_ |
					BREAK2_ |
					CONTINUE_ |
					CONTINUE2_ => {
						let a = &as_ref__!(env.jump_).a_;
						for j in a.iter().rev() {
							if *j == s {
								i = 0;
								break;
							}
						}
					}
					_ => 
					if self.b__() {
						if s.is_empty() {
							i = 0;
							break;
						}
						if let Some(i1) = t_::s2n__(&s) {
							i = i1;
							s.clear();
							break;
						}
					}
				}
			}
			if up > 0 && q_i < 0 {
				q_i = as_ref__!(env.q).i_
			}
		} else {
			match self.i__() {
				RETURN_ => q_i = as_ref__!(env.q).i_,
				_ => if self.b__() { i = 0; }
			}
		}
		match self.i__() {
			RETURN_ => if q_i >= up { i = q_i + 1 - up; }
			_ => {}
		}
		result2_::err5__(if i == NO_ {result2_::ERR_} else {self.i__()}, i, s)
	}
}
