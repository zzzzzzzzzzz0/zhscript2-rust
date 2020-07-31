use super::*;

pub struct Env_<'a> {
	pub fa:&'a FA_, pub gd:Opt_, pub q:qv_::T_, pub w:world_::T_,
	//pub wm:&'a mut WorldMut_,
	//pub ret:&'a mut result_::List_,
}

impl<'a> Env_<'a> {
	pub fn new(q:qv_::T_, w:world_::T_) -> Self {
		Self {fa:&None, gd:Default::default(), q, w}
	}
	pub fn new2(q:qv_::T_, env:&'a Self) -> Self {
		Self {q, w:env.w.clone(), ..*env}
	}
	pub fn new3(gd:Opt_, env:&'a Self) -> Self {
		Self {gd, q:env.q.clone(), w:env.w.clone(), ..*env}
	}
	#[allow(dead_code)]
	pub fn new4(fa:&'a FA_, q:qv_::T_, env:&'a Self) -> Self {
		Self {fa, q, w:env.w.clone(), ..*env}
	}
	pub fn new5(q:qv_::T_, gd:Opt_, env:&'a Self) -> Self {
		Self {gd, q, w:env.w.clone(), ..*env}
	}
}