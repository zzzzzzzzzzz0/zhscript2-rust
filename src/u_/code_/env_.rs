use super::*;

pub struct Env_ {
	pub fa:FA_, pub gd:Opt_, pub q:qv_::T_, pub w:world_::T_,
	pub ret:T_<result_::List_>,
}

impl<'a> Env_ {
	pub fn new(q:qv_::T_, w:world_::T_, ret:T_<result_::List_>) -> Self {
		Self {fa:attr_::i__(None), gd:Default::default(), q, w, ret}
	}
	pub fn new2(q:qv_::T_, env:&'a Self) -> Self {
		Self {fa:env.fa.clone(), q, w:env.w.clone(), ret:env.ret.clone(), ..*env}
	}
	#[allow(dead_code)]
	pub fn new9(q:qv_::T_, ret:T_<result_::List_>, env:&'a Self) -> Self {
		Self {fa:env.fa.clone(), q, w:env.w.clone(), ret, ..*env}
	}
	pub fn new3(gd:Opt_, env:&'a Self) -> Self {
		Self {fa:env.fa.clone(), gd, q:env.q.clone(), w:env.w.clone(), ret:env.ret.clone()}
	}
	pub fn new7(gd:Opt_, ret:T_<result_::List_>, env:&'a Self) -> Self {
		Self {fa:env.fa.clone(), gd, q:env.q.clone(), w:env.w.clone(), ret}
	}
	#[allow(dead_code)]
	pub fn new4(fa:FA_, q:qv_::T_, env:&'a Self) -> Self {
		Self {fa, q, w:env.w.clone(), ret:env.ret.clone(), ..*env}
	}
	pub fn new5(gd:Opt_, q:qv_::T_, env:&'a Self) -> Self {
		Self {fa:env.fa.clone(), gd, q, w:env.w.clone(), ret:env.ret.clone()}
	}
	pub fn new6(ret:T_<result_::List_>, env:&'a Self) -> Self {
		Self {fa:env.fa.clone(), ret, q:env.q.clone(), w:env.w.clone(), ..*env}
	}
	#[allow(dead_code)]
	pub fn new10(env:&'a Self) -> Self {
		Self {fa:env.fa.clone(), q:env.q.clone(), w:env.w.clone(), ret:env.ret.clone(), ..*env}
	}
}