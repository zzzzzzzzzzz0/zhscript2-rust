use super::{*};

pub const TOP_:&str = "顶";
pub const UP_:&str = "上";
pub const IN_:&str = "所";

fn z__(rem:&str, is_has:bool, mut shou:impl FnMut(&str) -> bool, in_q:Option<qv_::T_>, q:qv_::T_, w:world_::T_)
		-> Result<Option<qv_::T_>, Result2_> {
	let mut q2 = Some(q);
	match rem {
		TOP_ =>
			q2 = Some(as_ref__!(w).top_q_.clone()),
		UP_ => {
			q2 = match &as_ref__!(q2.unwrap()).up_ {
				Some(q3) => Some(q3.clone()),
				None => {
					if is_has {None} else {
						return Err(result2_::err2__("无上"))
					}
				}
			}
		}
		IN_ =>
			if let Some(q3) = in_q {
				q2 = Some(q3.clone())
			} else {
				return Err(as_ref__!(w).no_rem2__(rem))
			}
		_ => {
			let ret2 = qv_::for__(q2.clone().unwrap(), w.clone(), |q3, _, _| {
				if as_ref__!(q3).name_.contains(&rem.to_string()) {
					q2 = Some(q3);
					Some(())
				} else {None}
			});
			if ret2.is_none() {
				for q3 in &as_ref__!(w).datas_ {
					if as_ref__!(q3).name_.contains(&rem.to_string()) {
						return Ok(Some(q3.clone()))
					}
				}
			}
			if ret2.is_none() && !shou(rem) {
				return Err(as_ref__!(w).no_rem2__(rem))
			}
		}
	}
	Ok(q2)
}
fn z2__(rems:&[String], is_has:bool, mut shou:impl FnMut(&str) -> bool, in_q:Option<qv_::T_>, q:qv_::T_, w:world_::T_)
		-> Result<Option<qv_::T_>, Result2_> {
			let mut q2 = Some(q);
	for rem in rems {
		match z__(rem.as_str(), is_has, &mut shou, in_q.clone(), q2.clone().unwrap(), w.clone()) {
			Ok(q3) => q2 = q3,
			e => return e
		}
	}
	Ok(q2)
}
pub fn hello2__(rems:&[String], is_has:bool, shou:impl FnMut(&str) -> bool, q:qv_::T_, w:world_::T_)
		-> Result<Option<qv_::T_>, Result2_> {
	z2__(rems, is_has, shou, None, q, w)
}
pub fn hello1__(rem:&str, shou:impl FnMut(&str) -> bool, q:qv_::T_, w:world_::T_)
		-> Result<Option<qv_::T_>, Result2_> {
	z__(rem, false, shou, None, q, w)
}
pub fn hello__(rems:&[String], shou:impl FnMut(&str) -> bool, in_q:Option<qv_::T_>, q:qv_::T_, w:world_::T_)
		-> Result<Option<qv_::T_>, Result2_> {
	z2__(rems, false, shou, in_q, q, w)
}
