use super::*;

pub const     BREAK_:i32 = 2101;
pub const  CONTINUE_:i32 = 2102;
pub const    BREAK2_:i32 = 2201;
pub const CONTINUE2_:i32 = 2202;
pub const    RETURN_:i32 = 2003;
pub const      QUIT_:i32 = 2004;

pub trait Item_ : code_::Item_ {
	fn i__(&self) -> i32;
	
	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:&mut World_, _ret:&mut result_::List_) -> Result2_ {
		let mut ret2 = result_::List_::new();
		t_::o__(&self.a__()).hello__(gd, q, w, &mut ret2)?;
		let v = ret2.to_vec__();
		Err((self.i__(), if !v.is_empty() {
			v[0].to_string()
		} else {
			"".to_string()
		}, "".to_string()))
	}
}
