use super::u_::*;

pub struct Item_ {
	super_:code_::Item1_,
	a_:code_::OL_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:code_::Item1_::new2(&kws.begin_block_, &kws.end_block_), a_:None}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn kw2__(&self) -> keyword_::ORI_ {self.super_.kw2__()}
	fn add__(&mut self, a:code_::List_) -> Result2_ {
		self.a_ = Some(a);
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}

	fn hello__(&self, env:&code_::Env_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		t_::o__(&self.a_).hello__(&code_::Env_::new3(code_::Opt_ {jvhao_:false, ..env.gd}, env), wm, ret)?;
		if wm.dbg_.lc_ {
			wm.dbg_.lc_kw__(t_::or__(&self.super_.kw2__()));
		}
		ok__()
	}
}
