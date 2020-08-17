use super::{*, super::{as_ref__}};

pub const     BREAK_:i32 = 2101;
pub const  CONTINUE_:i32 = 2102;
pub const    BREAK2_:i32 = 2201;
pub const CONTINUE2_:i32 = 2202;
pub const    RETURN_:i32 = 2003;
pub const      QUIT_:i32 = 2004;
//pub const BEGIN_:i32 = 2000;
//pub const   END_:i32 = 2999;

pub trait Item_ : code_::Item_ {
	fn i__(&self) -> i32;
	
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		let ret2 = t__(result_::List_::new());
		t_::o__(&self.a__()).hello__(&code_::Env_::new6(ret2.clone(), env))?;
		let v = as_ref__!(ret2).to_vec__();
		Err((self.i__(), if !v.is_empty() {
			v[0].to_string()
		} else {
			"".to_string()
		}, "".to_string()))
	}
}
