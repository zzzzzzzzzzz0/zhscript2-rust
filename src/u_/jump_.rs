use super::{*, super::{as_ref__}};

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
			if *i == jump_::BREAK_ && s.is_empty() {
				return (false, true);
			}
			if *i == jump_::CONTINUE_ && s.is_empty() {
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

pub trait Item_ : code_::Item_ {
	fn i__(&self) -> i32;
	fn b__(&self) -> bool;
	
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		let ret2 = t__(result_::List_::new());
		t_::o__(&self.a__()).hello__(&code_::Env_::new6(ret2.clone(), env))?;
		let v = as_ref__!(ret2).to_vec__();
		let mut s = String::new();
		let mut i = NO_;
		loop {
			if !v.is_empty() {
				s = v[0].to_string();
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
			} else if self.b__() {
				i = 0;
			}
			break;
		}
		result2_::err4__(self.i__(), i, s, "".to_string())
	}
}
