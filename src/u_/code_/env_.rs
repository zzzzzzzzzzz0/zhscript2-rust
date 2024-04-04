use super::*;

pub struct Env_ {
	pub fa:FA_, pub gd:Opt_, pub q:qv_::T_, pub w:world_::T_,
	pub ret:T_<result_::List_>,
	pub in_q_:Option<qv_::T_>,
}

impl<'a> Env_ {
	pub fn new(q:qv_::T_, w:world_::T_, ret:T_<result_::List_>) -> Self {
		Self {fa:attr_::i__(None), gd:Default::default(), q, w, ret, in_q_:None}
	}
	pub fn new2(q:qv_::T_, env:&'a Self) -> Self {
		Self::new12(env.in_q_.clone(), q, env)
	}
	pub fn new12(in_q_:Option<qv_::T_>, q:qv_::T_, env:&'a Self) -> Self {
		Self {fa:env.fa.clone(), q, w:env.w.clone(), ret:env.ret.clone(), in_q_, ..*env}
	}
	#[allow(dead_code)]
	pub fn new9(q:qv_::T_, ret:T_<result_::List_>, env:&'a Self) -> Self {
		Self {fa:env.fa.clone(), q, w:env.w.clone(), ret, in_q_:env.in_q_.clone(), ..*env}
	}
	pub fn new3(gd:Opt_, env:&'a Self) -> Self {
		Self::new7(gd, env.ret.clone(), env)
	}
	pub fn new7(gd:Opt_, ret:T_<result_::List_>, env:&'a Self) -> Self {
		Self {fa:env.fa.clone(), gd, q:env.q.clone(), w:env.w.clone(), ret, in_q_:env.in_q_.clone()}
	}
	#[allow(dead_code)]
	pub fn new4(fa:FA_, q:qv_::T_, env:&'a Self) -> Self {
		Self {fa, q, w:env.w.clone(), ret:env.ret.clone(), in_q_:env.in_q_.clone(), ..*env}
	}
	pub fn new5(gd:Opt_, q:qv_::T_, env:&'a Self) -> Self {
		Self::new11(gd, env.in_q_.clone(), q, env)
	}
	pub fn new11(gd:Opt_, in_q_:Option<qv_::T_>, q:qv_::T_, env:&'a Self) -> Self {
		Self {fa:env.fa.clone(), gd, q, w:env.w.clone(), ret:env.ret.clone(), in_q_}
	}
	pub fn new6(ret:T_<result_::List_>, env:&'a Self) -> Self {
		Self {fa:env.fa.clone(), ret, q:env.q.clone(), w:env.w.clone(), in_q_:env.in_q_.clone(), ..*env}
	}
	#[allow(dead_code)]
	pub fn new10(env:&'a Self) -> Self {
		Self {fa:env.fa.clone(), q:env.q.clone(), w:env.w.clone(), ret:env.ret.clone(), in_q_:env.in_q_.clone(), ..*env}
	}
}