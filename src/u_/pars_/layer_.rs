use super::*;

pub type RL_ = Rc_<Layer_>;

pub struct Layer_ {
	pub i_:i32,
	pub kw_:keyword_::ORI_,
	ret_0_:Ret0_,
	up_:Option<RL_>,
}

impl<'a> Layer_ {
	pub fn new(ret_0:Ret0_) -> Self {
		Self::new3(0, None, ret_0, None)
	}
	pub fn new2(c:Self) -> RL_ {
		Rc_::new(c)
	}
	pub fn new3(i_:i32, kw_:keyword_::ORI_, ret_0_:Ret0_, up_:Option<RL_>) -> Self {
		Layer_ {i_, kw_, ret_0_, up_}
	}

	pub fn i1__(&self, kw:keyword_::RI_, kws:&keywords_::A_, mut2:&mut Mut2_, w:&World_) -> bool {
		#[cfg(debug_assertions)]
		mut2.bp__("-l1-", true, w);
		let mut ret = self.i_;
		#[cfg(debug_assertions)]
		if mut2.lc__(w) {
			lc__!("({}{}{}/", kw.s_, ret, self.line__());
		}
		let mut must = false;
		match kw.id_ {
			keyword_::Id_::Jvhao |
			keyword_::Id_::Douhao => {
				self.for__(|c| {
					if let Some(kw) = &c.kw_ {
						#[cfg(debug_assertions)]
						if mut2.lc__(w) {
							lc__!("{:?} ", kw.id_);
						}
						match kw.id_ {
							//先
							//	。
							//了
							keyword_::Id_::BeginBlock => {
								ret = c.i_;
								//先
								//了
								//。
								if ret < self.i_ {
									must = true;
								}
								return true
							}
							_ => {
								//1 所有“可执行”语句、“如果”的定位符，一直返到无
								ret = c.i_ - 1;
							}
						}
					}
					//1
					false
				});
			}
			keyword_::Id_::EndBlock => {
				let mut b = false;
				self.for__(|c| {
					if let Some(kw) = &c.kw_ {
						if b {
							//做为了这些的定尾符
							match kw.id_ {
								keyword_::Id_::If => {
									if ret_0__(&c.ret_0_) == 3 {
										ret = c.i_ - 1;
									}
								}
								keyword_::Id_::Switch |
								keyword_::Id_::Switch2 |
								keyword_::Id_::For |
								keyword_::Id_::Range => {
									ret = c.i_ - 1;
								}
								_ => {}
							}
							return true
						}
						//仅匹配（上）一个
						if kw.id_ == keyword_::Id_::BeginBlock {
							ret = c.i_ - 1;
							b = true;
						}
					} else if b {
						return true
					}
					false
				});
			}
			keyword_::Id_::BeginBlock => {
				let mut b = false;
				self.for__(|c| {
					if let Some(kw2) = &c.kw_ {
						match kw2.id_ {
							keyword_::Id_::Switch |
							keyword_::Id_::Switch2 => if b {
								must = true;
								mut2.u_ = kw.s_.len();
								return true
							}
							keyword_::Id_::BeginBlock => {
								return true
							}
							_ => b = true,
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
					let mut ret2 = -1;
					self.for__(|c| {
						if let Some(kw) = &c.kw_ {
							match kw.id_ {
								keyword_::Id_::If => {
									if ret_0__(&c.ret_0_) == 1 {
										ret = if ret2 != -1 {ret2} else {c.i_};
										return true
									}
								}
								keyword_::Id_::BeginBlock => {
									ret2 = c.i_;
								}
								_ => {}
							}
						}
						false
					});
				}
				else if kw.g_.if2_ || kws.if2_.contains(&kw) {
					self.for__(|c| {
						if let Some(kw2) = &c.kw_ {
							if let keyword_::Id_::If = kw2.id_ {
								ret = c.i_;
								return true
							}
						}
						false
					});
				}
			}
		}
		#[cfg(debug_assertions)]
		if mut2.lc__(w) {
			lc__!("{}{})\n", ret, t_::b__(must));
		}
		if ret < self.i_ {
			mut2.i_ = ret;
			mut2.kw_ = Some(kw);
			return true
		}
		if must {
			mut2.kw_ = Some(kw);
			return true
		}
		false
	}

	#[cfg(debug_assertions)]
	fn line__(&self) -> String {
		let mut ret = String::new();
		self.for__(|c| {
			ret.push(' ');
			if let Some(kw) = &c.kw_ {
				ret += &kw.s_;
			}
			ret += &c.i_.to_string();
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
