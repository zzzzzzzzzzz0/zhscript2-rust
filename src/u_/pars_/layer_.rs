use super::*;

pub struct LayerMut_ {
	pub kw_:keyword_::ORI_,
	pub i_:i32,
}
impl LayerMut_ {
	pub fn new() -> Self {Self{kw_:None, i_:0}}
}

pub struct LayerMut2_ {
	pub block_:i32,
	pub case_:i32,
}
impl LayerMut2_ {
	pub fn new() -> Self {Self{block_:0, case_:0}}
}

pub type RL_ = Rc_<Layer_>;

pub struct Layer_ {
	pub i_:i32,
	pub kw_:keyword_::ORI_,
	pub block_:i32,
	pub block2_:i32,
	up_:Option<RL_>,
}

impl<'a> Layer_ {
	pub fn new() -> Self {
		Self::new3(0, 0, None, None)
	}
	pub fn new2(c:Self) -> RL_ {
		Rc_::new(c)
	}
	pub fn new3(i_:i32, block2_:i32, kw_:keyword_::ORI_, up_:Option<RL_>) -> Self {
		Layer_ {i_, kw_, block_:0, block2_, up_}
	}

	pub fn i1__(&self, kw:keyword_::RI_, mut2:&LayerMut2_, kws:&keywords_::A_, dbg:&Dbg_) -> i32 {
		if dbg.par_lc_ {
			lc__!("(b{}{})", mut2.block_, self.line__());
		}
		let mut ret = self.i_;
		let mut must = false;
		match kw.id_ {
			keyword_::Id_::Jvhao |
			keyword_::Id_::Douhao => {
				let mut b = false;
				self.for__(|c| {
					if b {
						ret = c.i_;
						if let Some(kw) = &c.kw_ {
							if dbg.par_lc_ {
								lc6__!("{:?} ", kw.id_);
							}
							match kw.id_ {
								keyword_::Id_::Expl |
								keyword_::Id_::If |
								keyword_::Id_::Set => {
									return false
								}
								_ => {}
							}
						}
						return true
					}
					if let Some(kw) = &c.kw_ {
						if dbg.par_lc_ {
							lc__!("{:?} ", kw.id_);
						}
						match kw.id_ {
							keyword_::Id_::BeginBlock => {
								if mut2.block_ == c.block2_ {
									ret = c.i_;
									return true
								}
							}
							keyword_::Id_::If => {
								b = true;
							}
							_ => {
								if kw.g_.print_ || kws.print_.contains(kw) {
									b = true;
									must = true;
								}
							}
						}
						return false
					}
					true
				});
			}
			keyword_::Id_::EndBlock => {
				let mut b = false;
				self.for__(|c| {
					if let Some(kw) = &c.kw_ {
						if b {
							match kw.id_ {
								keyword_::Id_::For |
								keyword_::Id_::Switch |
								keyword_::Id_::If |
								keyword_::Id_::Print => {
									ret = c.i_;
									return false
								}
								keyword_::Id_::BeginBlock => {
									if c.block2_ == mut2.block_ {
										ret = c.i_;
										return true
									}
								}
								_ => {}
							}
							return true
						}
						if kw.id_ == keyword_::Id_::BeginBlock {
							ret = c.i_;
							must = true;
							if c.block2_ == mut2.block_ {
								return true
							}
							b = true;
						}
					}
					false
				});
			}
			keyword_::Id_::Equ => {
				self.for__(|c| {
					if let Some(kw) = &c.kw_ {
						if kw.g_.set_ || kws.set_.contains(kw) {
							ret = c.i_;
							return true
						}
					}
					false
				});
			}
			_ => {
				if kw.g_.if_ || kws.if_.contains(&kw) {
					self.for__(|c| {
						if let Some(kw) = &c.kw_ {
							match kw.id_ {
								keyword_::Id_::If |
								keyword_::Id_::BeginBlock => {
									ret = c.i_;
									return true
								}
								_ => {}
							}
						}
						false
					});
				}
				else if kw.g_.if2_ || kws.if2_.contains(&kw) {
					self.for__(|c| {
						if let Some(kw) = &c.kw_ {
							match kw.id_ {
								keyword_::Id_::If |
								keyword_::Id_::BeginBlock => {
									ret = c.i_;
									return true
								}
								_ => {}
							}
						}
						false
					});
				}
			}
		}
		let ret2 = if must || ret < self.i_ {ret} else {-1};
		if dbg.par_lc_ {
			lc__!("{}{} {})\n", ret, t_::b__(must), ret2);
		}
		ret2
	}

	pub fn line__(&self) -> String {
		let mut ret = String::new();
		self.for__(|c| {
			ret.push(' ');
			if let Some(kw) = &c.kw_ {
				ret += &kw.s_;
			}
			ret += &c.i_.to_string();
			ret.push(',');
			ret += &c.block_.to_string();
			ret.push_str("/b");
			ret += &c.block2_.to_string();
			false
		});
		ret
	}

	fn for__(&self, mut f:impl FnMut(&Self) -> bool) {
		let mut c = self;
		loop {
			if f(c) {break}
			if let Some(up) = &c.up_ {c = up} else {break}
		}
	}
}
