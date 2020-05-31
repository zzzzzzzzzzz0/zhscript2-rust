use super::{*, super::*};

mod layer_;
use layer_::*;

cfg_if! {
	if #[cfg(feature = "thread")] {

pub trait U11_ : Sync + Send {
	fn u11__(&self, kw:keyword_::RI_, kws:&keyword_::List_, codes:& code_::List_) -> code_::OI_;
}

	} else {

pub trait U11_ {
	fn u11__(&self, kw:keyword_::RI_, kws:&keyword_::List_, codes:& code_::List_) -> code_::OI_;
}

	}
}

#[derive(Clone, Default)]
pub struct Pars_ {
	pub u11_:Option<Rc_<dyn U11_>>,
}

type Ret_<'a> = (&'a mut i32,
	       &'a mut code_::List_,
	Option<&'a mut code_::List_>,
	Option<&'a mut code_::List_>);

impl Pars_ {
	fn is_kw__(&self, a3:&[u8], a2:&[u8], i:&mut usize) -> &str {
		let mut i3 = 0;
		let mut i4 = *i;
		loop {
			if i3 >= a3.len() {
				*i = i4;
				return "l1"
			}
			if i4 >= a2.len() {break}
			let c = a2[i4];
			i4 += 1;
			if IsText_::sp__(c) {
				if i3 == 0 {return "forkw"}
				continue
			}
			if c != a3[i3] {break}
			i3 += 1;
		}
		""
	}
	
	#[allow(clippy::cognitive_complexity)]
	#[allow(clippy::too_many_arguments)]
	fn hello2__(&self, a2:&[u8], i:&mut usize, layer:RL_, layer_mut:&mut LayerMut_, layer_mut2:&mut LayerMut2_,
			is:&mut IsText_, kws:&keywords_::A_, ret:&mut Ret_, w:&World_, dbg:&Dbg_) -> Result2_ {
		if dbg.par_lc_ {
			lc__!("({} {})", layer.i_, i);
		}
		let len2 = a2.len();
		let mut text = vec![];
		'l1: loop {
			if *i >= len2 {
				break
			}
			let mut kw2 = vec![];
			#[allow(clippy::never_loop)]
			'l2: loop {
				let mut can_a2 = false;
				{
					let a = if is.var__() {
						&kws.var_
					} else if is.text__() {
						&kws.text_
					} else if is.code__() {
						&kws.code_
					} else if is.yuanyang__() {
						&kws.yuanyang_
					} else if is.rem__() {
						&kws.rem_
					} else if is.rem2__() {
						&kws.rem2_
					} else if is.text2__() {
						&kws.text2_
					} else {
						can_a2 = true;
						&w.kws_.a_
					};
					'forkw: for kw in a {
						match self.is_kw__(kw.s_.as_bytes(), a2, i) {
							"l1" => {kw2.push(kw.clone()); break 'l2}
							"forkw" => break 'forkw,
							_ => {}
						}
					}
				}
				if can_a2 {
					'forkw2: for kw in &w.kws_.a2_ {
						match self.is_kw__(kw.s_.as_bytes(), a2, i) {
							"l1" => {
								for id in &kw.id_ {
									if let Some(kw3) = w.kws_.a_.iter().find(|i| i.id_ == *id) {
										kw2.push(kw3.clone());
									}
								}
								break 'l2
							}
							"forkw" => break 'forkw2,
							_ => {}
						}
					}
				}
				break
			}
			#[allow(clippy::never_loop)]
			for kw in &kw2 {
				if self.add_text_kw__(kw, is, &mut text, dbg) {
					continue
				}
				match kw.id_ {
					keyword_::Id_::BeginRem2 |
					keyword_::Id_::EndRem2 |
					keyword_::Id_::BeginVar |
					keyword_::Id_::EndVar => {
						if is.as2__() {
							if dbg.par_lc_ {
								dbg.lc2__(kw, &kw.s_);
							}
							self.add_kw2__(kw, &mut text);
							continue
						}
					}
					_ => {}
				}
				self.add_text__(&mut text, is, ret, w, dbg)?;
				if dbg.par_lc_ {
					dbg.lc2__(kw, &kw.s_);
				}
				match kw.id_ {
					keyword_::Id_::EndBlock =>
						layer_mut2.block_ -= 1,
					keyword_::Id_::Equ |
					keyword_::Id_::Then |
					keyword_::Id_::Else => {
						*ret.0 = match kw.id_ {
							keyword_::Id_::Else => 3,
							_ => 2
						};
					}
					keyword_::Id_::Maohao =>
							continue,
					_ => {}
				}
				if match kw.id_ {
					keyword_::Id_::Jvhao |
					keyword_::Id_::EndBlock |
					keyword_::Id_::Equ => true,
					_ => false,
				} || kw.g_.if_ || kw.g_.if2_ || kws.if_.contains(kw) || kws.if2_.contains(kw) {
					let i1 = layer.i1__(kw.clone(), layer_mut2, kws, dbg);
					if i1 >= 0 {
						layer_mut.i_ = i1;
						layer_mut.kw_ = Some(kw.clone());
						break 'l1
					}
				}
				if match kw.id_ {
					keyword_::Id_::Jvhao |
					keyword_::Id_::Dunhao |
					keyword_::Id_::Brkpoint => true,
					_ => false
				} || kw.g_.if_ || kws.if_.contains(kw) {
					self.add_kw__(&kw, ret, w)?;
					continue
				}
				match kw.id_ {
					keyword_::Id_::BeginBlock =>
						layer_mut2.block_ += 1,
					keyword_::Id_::EndBlock |
					keyword_::Id_::Equ |
					keyword_::Id_::Then |
					keyword_::Id_::Else =>
						continue,
					keyword_::Id_::BeginRem2 =>
						is.rem22__(),
					keyword_::Id_::EndRem2 => {
						is.rem23__();
						break 'l1
					}
					keyword_::Id_::BeginVar =>
						is.var2__(),
					keyword_::Id_::EndVar => {
						is.var3__();
						break 'l1
					}
					keyword_::Id_::Switch =>
						layer_mut2.case_ += 1,
					keyword_::Id_::ParBrkpoint => {
						#[allow(non_snake_case)]
						#[allow(unused_variables)]
						let o_H_o = true;
						continue
					}
					_ => {}
				}
				let mut ret2_2 = None;
				let mut ret2_3 = None;
				let mut ret2_2t = code_::List_::new();
				let mut ret2_3t = code_::List_::new();
				if dbg.par_lc_ {
					lc__!("({}", layer.i_);
				}
				let mut layer2 = Layer_::new3(layer.i_ + 1, layer_mut2.block_,
					Some(kw.clone()), Some(layer.clone()));
				match kw.id_ {
					keyword_::Id_::For | keyword_::Id_::Range | keyword_::Id_::Switch =>
						layer2.block_ = 1,
					keyword_::Id_::If =>
						layer2.block_ = 3,
					_ => {}
				};
				match kw.id_ {
					keyword_::Id_::If => {
						ret2_2 = Some(&mut ret2_2t);
						ret2_3 = Some(&mut ret2_3t);
					}
					_ => {
						if kw.g_.set_ || kws.set_.contains(kw) {
							ret2_2 = Some(&mut ret2_2t)
						}
					}
				};
				let mut ret2_0 = 1;
				let mut ret2_1 = code_::List_::new();
				let layer2 = Layer_::new2(layer2);
				let mut layer2_mut = LayerMut_::new();
				self.hello2__(a2, i, layer2.clone(), &mut layer2_mut, layer_mut2,
					is, kws, &mut (&mut ret2_0, &mut ret2_1, ret2_2, ret2_3), w, dbg)?;
				if kw.id_ == keyword_::Id_::Switch {
					layer_mut2.case_ -= 1
				}
				if dbg.par_lc_ {
					lc__!("(");
					lc__!("{}{}codes[{}])", kw.s_,ret.0, ret2_1.len());
					if !ret2_2t.is_empty() {
						lc__!("2[{}])", ret2_2t.len());
					}
					if !ret2_3t.is_empty() {
						lc__!("3[{}])", ret2_3t.len());
					}
					println!();
				}
				if let Some(code2) = self.code2__(kw.clone(), &ret2_1, w) {
					{
						let mut r = as_mut_ref__!(code2);
						w.ret4__(r.add__(ret2_1), kw)?;
						r.add2__(ret2_2t)?;
						r.add3__(ret2_3t)?;
					}
					self.add_code__(code2, ret)?;
				} else {return result2_::err__([&w.text__(&kw.s_), "没实现"].concat())}
				if let Some(kw) = &layer2_mut.kw_ {
					if dbg.par_lc_ {
						lc__!("({}{}ceng{}>{} {}={})", kw.s_, ret2_0, layer.i_, layer2_mut.i_, layer.block_, layer_mut2.block_);
					}
					*ret.0 = ret2_0;
					if layer.i_ > layer2_mut.i_ {
						layer_mut.kw_ = layer2_mut.kw_.clone();
						layer_mut.i_ = layer2_mut.i_;
						if dbg.par_lc_ {
							println!();
						}
						break 'l1
					}
					if kw.id_ == keyword_::Id_::EndBlock && (layer.block_ == layer_mut2.block_ + 1
						|| (layer.block_ > 0 && layer.block_ == layer_mut2.block_)
					) {
						if dbg.par_lc_ {
							lc__!("{}+1)\n", layer_mut2.block_);
						}
						break 'l1
					}
					if match kw.id_ {
						keyword_::Id_::Jvhao | keyword_::Id_::Douhao => true,
						_ => false
					} || kw.g_.if_ || kws.if_.contains(kw) {
						*ret.0 = 1;
						self.add_kw__(&kw, ret, w)?;
					}
				}
				continue
			}
			if !kw2.is_empty() {continue 'l1}
			let c = a2[*i];
			if is.add__(c) {
				if is.need_clear__() {
					self.add_text__(&mut text, is, ret, w, dbg)?;
				}
				text.push(c);
			}
			*i += 1
		}
		self.add_text__(&mut text, is, ret, w, dbg)
	}
	pub fn hello__(&self, s:&str, fit:impl Fn(&mut IsText_), codes:&mut code_::List_, w:&World_, dbg:&Dbg_) -> Result2_ {
		let a2 = s.as_bytes();
		let mut i = 0;
		let mut is = Default::default();
		fit(&mut is);
		let kws = keywords_::A_::new(&w.kws_);
		if dbg.par_lc_ {
			println!()
		}
		let ret = w.ret3__(self.hello2__(a2, &mut i, Layer_::new2(Layer_::new()),
			&mut LayerMut_::new(), &mut LayerMut2_::new(),
			&mut is, &kws, &mut (&mut 1, codes, None, None), w, dbg), a2, 0, i);
		if dbg.par_lc_ {
			println!()
		}
		ret
	}
	
	fn add_text_kw__(&self, kw:&keyword_::Item_, is:&mut IsText_, text:&mut Vec<u8>, dbg:&Dbg_) -> bool {
		if let add@1..=2 = match kw.id_ {
			keyword_::Id_::BeginRem			=> is.add3__("r+"),
			keyword_::Id_::EndRem			=> is.add3__("r-"),
			keyword_::Id_::BeginText		=> is.add2__("t+"),
			keyword_::Id_::EndText			=> is.add2__("t-"),
			keyword_::Id_::BeginYuanyang	=> is.add2__("y+"),
			keyword_::Id_::EndYuanyang		=> is.add2__("y-"),
			keyword_::Id_::BeginCode		=> is.add2__("c+"),
			keyword_::Id_::EndCode			=> is.add2__("c-"),
			keyword_::Id_::BeginText2		=> is.add2__("e+"),
			keyword_::Id_::EndText2			=> is.add2__("e-"),
			_ => 0,
		} {
			if dbg.par_lc_ {
				//lc__!("(add{})",add);
			}
			if add == 2 {
				self.add_kw2__(kw, text);
			}
			if dbg.par_lc_ {
				dbg.lc2__(kw, &kw.s_);
			}
			true
		} else {
			false
		}
	}
	fn add_kw2__(&self, kw:&keyword_::Item_, text:&mut Vec<u8>) {
		for &i in kw.s_.as_bytes() {
			text.push(i);
		}
	}
	
	fn add_kw__(&self, kw:&keyword_::RI_, ret:&mut Ret_, w:&World_) -> Result2_ {
		self.add_code__(code_::i__(code_::Item1_::new(
			//kw
			match kw.id_ {
				keyword_::Id_::Jvhao => &w.kws_.jvhao_,
				keyword_::Id_::Dunhao => &w.kws_.dunhao_,
				keyword_::Id_::Brkpoint => &w.kws_.brkpoint_,
				keyword_::Id_::Dengyu => &w.kws_.dengyu_,
				keyword_::Id_::Xiaoyudengyu => &w.kws_.xiaoyudengyu_,
				keyword_::Id_::Xiaoyu => &w.kws_.xiaoyu_,
				keyword_::Id_::Dayudengyu => &w.kws_.dayudengyu_,
				keyword_::Id_::Dayu => &w.kws_.dayu_,
				keyword_::Id_::Not => &w.kws_.not_,
				keyword_::Id_::And => &w.kws_.and_,
				keyword_::Id_::Or => &w.kws_.or_,
				_ => panic!("{} {:?}", kw.s_, kw.id_),
			}
		)), ret)
	}

	fn code2__(&self, kw:keyword_::RI_, codes:&code_::List_, w:&World_) -> code_::OI_ {
		match kw.id_ {
			keyword_::Id_::BeginRem2 =>
				rem2_::new(&w.kws_, codes),
			keyword_::Id_::BeginVar =>
				super::super::var_::new(&w.kws_, codes),
			keyword_::Id_::BeginBlock =>
				code_::oi__(block_::Item_::new(&w.kws_)),
			keyword_::Id_::For =>
				code_::oi__(for_::Item_::new(&w.kws_)),
			keyword_::Id_::Break =>
				code_::oi__(break_::Item_::new(&w.kws_)),
			keyword_::Id_::Continue =>
				code_::oi__(continue_::Item_::new(&w.kws_)),
			keyword_::Id_::Range =>
				code_::oi__(range_::Item_::new(&w.kws_)),
			keyword_::Id_::Break2 =>
				code_::oi__(break2_::Item_::new(&w.kws_)),
			keyword_::Id_::Continue2 =>
				code_::oi__(continue2_::Item_::new(&w.kws_)),
			keyword_::Id_::Return =>
				code_::oi__(return_::Item_::new(&w.kws_)),
			keyword_::Id_::Quit =>
				code_::oi__(quit_::Item_::new(&w.kws_)),
			keyword_::Id_::If =>
				code_::oi__(if_::Item_::new(&w.kws_)),
			keyword_::Id_::Switch =>
				code_::oi__(switch_::Item_::new(&w.kws_)),
			keyword_::Id_::Guandaodu =>
				code_::oi__(guandaodu_::Item_::new(&w.kws_)),
			keyword_::Id_::Guandaojie =>
				code_::oi__(guandaojie_::Item_::new(&w.kws_)),
			keyword_::Id_::Mod =>
				code_::oi__(mod_::Item_::new(&w.kws_)),
			keyword_::Id_::ModFree =>
				code_::oi__(mod_free_::Item_::new(&w.kws_)),
			keyword_::Id_::Name =>
				code_::oi__(name_::Item_::new(&w.kws_)),
			keyword_::Id_::Set =>
				code_::oi__(set_::Item_::new(&w.kws_)),
			keyword_::Id_::Alias =>
				code_::oi__(alias_::Item_::new(&w.kws_)),
			keyword_::Id_::Def =>
				code_::oi__(super::super::def_::Item_::new(&w.kws_)),
			keyword_::Id_::Has =>
				code_::oi__(has_::Item_::new(&w.kws_)),
			keyword_::Id_::Eval =>
				code_::oi__(super::super::eval_::Item_::new(&w.kws_)),
			keyword_::Id_::Load =>
				code_::oi__(load_::Item_::new(&w.kws_)),
			keyword_::Id_::Print =>
				code_::oi__(print_::Item_::new(&w.kws_)),
			keyword_::Id_::Expl =>
				code_::oi__(super::super::expl_::Item_::new(&w.kws_)),
			keyword_::Id_::Exec =>
				code_::oi__(exec_::Item_::new(&w.kws_)),
			keyword_::Id_::U11 => {
				if let Some(u11) = &self.u11_ {
					u11.u11__(kw, &w.kws_, codes)
				} else {
					None
				}
			}
			_ => None,
		}
	}

	fn add_text__(&self, text:&mut Vec<u8>, is:&mut IsText_, ret:&mut Ret_, w:&World_, dbg:&Dbg_) -> Result2_ {
		if !text.is_empty() {
			let s = String::from_utf8(text.clone()).unwrap();
			let code:code_::I_ =
				if is.undef__() {
					code_::i__(undef_::Item_::new(&w.kws_, &s))
				} else {
					code_::i__(text_::Item_::new(&w.kws_, &s))
				};
			if dbg.par_lc_ {
				lc4__!("{}", s);
			}
			text.clear();
			is.clear__();
			self.add_code__(code, ret)
		} else {
			ok__()
		}
	}
	fn add_code__(&self, code:code_::I_, ret:&mut Ret_) -> Result2_ {
		match ret.0 {
			2 => {
				if let Some(ls) = &mut ret.2 {
					ls.add__(code)
				} else {
					return result2_::err2__("缺少后句")
				}
			},
			3 => {
				if let Some(ls) = &mut ret.3 {
					ls.add__(code)
				} else {
					return result2_::err2__("缺少第二后句")
				}
			},
			_ => ret.1.add__(code),
		}
		ok__()
	}
}
