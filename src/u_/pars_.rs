use super::*;
use super::super::*;
use super::is_text_::*;

mod layer_;
use layer_::*;

pub struct Pars_ {}

type Ret_<'a> = (&'a mut i32,
	       &'a mut code_::List_,
	Option<&'a mut code_::List_>,
	Option<&'a mut code_::List_>);

impl Pars_ {
	#[allow(clippy::cognitive_complexity)]
	#[allow(clippy::too_many_arguments)]
	fn hello2__(&self, a2:&[u8], i:&mut usize, layer:RL_, layer_mut:&mut LayerMut_, layer_mut2:&mut LayerMut2_,
			is:&mut IsText_, kws:&keywords_::A_, ret:&mut Ret_, w:&World_) -> Result2_ {
		if w.dbg_.par_lc_ {
			lc__!("({} {})", layer.i_, i);
		}
		let len2 = a2.len();
		let mut text = vec![];
		'l1: loop {
			if *i >= len2 {
				break
			}
			let a = if is.text__() {
				&kws.text_
			} else if is.code__() {
				&kws.code_
			} else if is.yuanyang__() {
				&kws.yuanyang_
			} else if is.rem__() {
				&kws.rem_
			} else if is.rem2__() {
				&kws.rem2_
			} else if is.var__() {
				&kws.var_
			} else {
				&w.kws_.a_
			};
			'forkw: for kw in a {
				let a3 = kw.s_.as_bytes();
				let mut i3 = 0;
				let mut i4 = *i;
				loop {
					if i3 >= a3.len() {
						*i = i4;
						if self.add_text_kw__(kw, is, &mut text, w) {
							continue 'l1
						}
						match kw.id_ {
							keyword_::Id_::BeginRem2 |
							keyword_::Id_::EndRem2 |
							keyword_::Id_::BeginVar |
							keyword_::Id_::EndVar => {
								if is.as2__() {
									if w.dbg_.par_lc_ {
										w.dbg_.lc2__(kw, &kw.s_);
									}
									self.add_kw_2__(kw, &mut text);
									continue 'l1
								}
							}
							_ => {}
						}
						self.add_text__(&mut text, is, ret, w)?;
						if w.dbg_.par_lc_ {
							w.dbg_.lc2__(kw, &kw.s_);
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
									continue 'l1,
							_ => {}
						}
						if match kw.id_ {
							keyword_::Id_::Jvhao |
							keyword_::Id_::EndBlock |
							keyword_::Id_::Equ => true,
							_ => false,
						} || kw.g_.if_ || kw.g_.if2_ {
							let i1 = layer.i1__(kw.clone(), layer_mut2, w);
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
						} || kw.g_.if_ {
							self.add_code2__(code_::Item1_::new(&kw), ret)?;
							continue 'l1
						}
						match kw.id_ {
							keyword_::Id_::BeginBlock =>
								layer_mut2.block_ += 1,
							keyword_::Id_::EndBlock |
							keyword_::Id_::Equ |
							keyword_::Id_::Then |
							keyword_::Id_::Else =>
								continue 'l1,
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
								continue 'l1
							}
							_ => {}
						}
						let mut ret2_2 = None;
						let mut ret2_3 = None;
						let mut ret2_2t = code_::List_::new();
						let mut ret2_3t = code_::List_::new();
						if w.dbg_.par_lc_ {
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
								if kw.g_.set_ {
									ret2_2 = Some(&mut ret2_2t)
								}
							}
						};
						let mut ret2_0 = 1;
						let mut ret2_1 = code_::List_::new();
						let layer2 = Layer_::new2(layer2);
						let mut layer2_mut = LayerMut_::new();
						self.hello2__(a2, i, layer2.clone(), &mut layer2_mut, layer_mut2,
							is, kws, &mut (&mut ret2_0, &mut ret2_1, ret2_2, ret2_3), w)?;
						if kw.id_ == keyword_::Id_::Switch {
							layer_mut2.case_ -= 1
						}
						if w.dbg_.par_lc_ {
							lc__!("(");
							/*if let Some(kw) = &layer.kw_ {
								lc__!("({})", kw.s_);
							}*/
							lc__!("{}{}codes[{}])", kw.s_,ret.0, ret2_1.len());
							if !ret2_2t.is_empty() {
								lc__!("2[{}])", ret2_2t.len());
							}
							if !ret2_3t.is_empty() {
								lc__!("3[{}])", ret2_3t.len());
							}
							println!();
						}
						if let Some(mut code2) = self.code2__(&kw, &ret2_1, w) {
							w.ret4__(code2.add__(ret2_1), code2.kw__())?;
							code2.add2__(ret2_2t)?;
							code2.add3__(ret2_3t)?;
							self.add_code__(code2, ret)?;
						} else {return result2_::err__([&w.text__(&kw.s_), "没实现"].concat())}
						if let Some(kw) = &layer2_mut.kw_ {
							if w.dbg_.par_lc_ {
								lc__!("({}{}ceng{}>{})", kw.s_, ret2_0, layer.i_, layer2_mut.i_);
							}
							*ret.0 = ret2_0;
							if layer.i_ > layer2_mut.i_ {
								layer_mut.kw_ = layer2_mut.kw_.clone();
								layer_mut.i_ = layer2_mut.i_;
								if w.dbg_.par_lc_ {
									println!();
								}
								break 'l1
							}
							if kw.id_ == keyword_::Id_::EndBlock && layer.block_ == layer_mut2.block_ + 1 {
								if w.dbg_.par_lc_ {
									lc__!("{}+1)\n", layer_mut2.block_);
								}
								break 'l1
							}
							if match kw.id_ {
								keyword_::Id_::Jvhao | keyword_::Id_::Douhao => true,
								_ => false
							} || kw.g_.if_ {
								*ret.0 = 1;
								self.add_code2__(code_::Item1_::new(&kw), ret)?;
							}
						}
						continue 'l1
					}
					if i4 >= len2 {break}
					let c = a2[i4];
					i4 += 1;
					if IsText_::sp__(c) {
						if i3 == 0 {break 'forkw}
						continue
					}
					if c != a3[i3] {break}
					i3 += 1;
				}
			}
			let c = a2[*i];
			if is.add__(c) {
				if is.need_clear__() {
					self.add_text__(&mut text, is, ret, w)?;
				}
				text.push(c);
			}
			*i += 1
		}
		self.add_text__(&mut text, is, ret, w)
	}
	pub fn hello__(&self, s:&str, codes:&mut code_::List_, w:&World_) -> Result2_ {
		let a2 = s.as_bytes();
		let mut i = 0;
		let mut is = IsText_::new();
		let kws = keywords_::A_::new(&w.kws_);
		if w.dbg_.par_lc_ {
			println!()
		}
		let ret = w.ret3__(self.hello2__(a2, &mut i, Layer_::new2(Layer_::new()),
			&mut LayerMut_::new(), &mut LayerMut2_::new(),
			&mut is, &kws, &mut (&mut 1, codes, None, None), w), a2, 0, i);
		if w.dbg_.par_lc_ {
			println!()
		}
		ret
	}
	
	fn add_text_kw__(&self, kw:&keyword_::Item_, is:&mut IsText_, text:&mut Vec<u8>, w:&World_) -> bool {
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
			if w.dbg_.par_lc_ {
				//lc__!("(add{})",add);
			}
			if add == 2 {
				self.add_kw_2__(kw, text);
			}
			if w.dbg_.par_lc_ {
				w.dbg_.lc2__(kw, &kw.s_);
			}
			true
		} else {
			false
		}
	}
	fn add_kw_2__(&self, kw:&keyword_::Item_, text:&mut Vec<u8>) {
		for &i in kw.s_.as_bytes() {
			text.push(i);
		}
	}

	fn code2__(&self, kw:&keyword_::Item_, codes:&code_::List_, w:&World_) -> Option<Box<dyn code_::Item_ + 'static>> {
		match kw.id_ {
			keyword_::Id_::BeginRem2 =>
				Some(rem2_::new(&w.kws_, &codes)),
			keyword_::Id_::BeginVar =>
				Some(super::super::var_::new(&w.kws_, &codes)),
			keyword_::Id_::BeginBlock =>
				Some(Box::new(block_::Item_::new(&w.kws_))),
			keyword_::Id_::For =>
				Some(Box::new(for_::Item_::new(&w.kws_))),
			keyword_::Id_::Break =>
				Some(Box::new(break_::Item_::new(&w.kws_))),
			keyword_::Id_::Continue =>
				Some(Box::new(continue_::Item_::new(&w.kws_))),
			keyword_::Id_::Range =>
				Some(Box::new(range_::Item_::new(&w.kws_))),
			keyword_::Id_::Break2 =>
				Some(Box::new(break2_::Item_::new(&w.kws_))),
			keyword_::Id_::Continue2 =>
				Some(Box::new(continue2_::Item_::new(&w.kws_))),
			keyword_::Id_::Return =>
				Some(Box::new(return_::Item_::new(&w.kws_))),
			keyword_::Id_::Quit =>
				Some(Box::new(quit_::Item_::new(&w.kws_))),
			keyword_::Id_::If =>
				Some(Box::new(if_::Item_::new(&w.kws_))),
			keyword_::Id_::Switch =>
				Some(Box::new(switch_::Item_::new(&w.kws_))),
			keyword_::Id_::Guandaodu =>
				Some(Box::new(guandaodu_::Item_::new(&w.kws_))),
			keyword_::Id_::Guandaojie =>
				Some(Box::new(guandaojie_::Item_::new(&w.kws_))),
			keyword_::Id_::Mod =>
				Some(Box::new(mod_::Item_::new(&w.kws_))),
			keyword_::Id_::Name =>
				Some(Box::new(name_::Item_::new(&w.kws_))),
			keyword_::Id_::Set =>
				Some(Box::new(set_::Item_::new(&w.kws_))),
			keyword_::Id_::Alias =>
				Some(Box::new(alias_::Item_::new(&w.kws_))),
			keyword_::Id_::Def =>
				Some(Box::new(super::super::def_::Item_::new(&w.kws_))),
			keyword_::Id_::Has =>
				Some(Box::new(has_::Item_::new(&w.kws_))),
			keyword_::Id_::Eval =>
				Some(Box::new(super::super::eval_::Item_::new(&w.kws_))),
			keyword_::Id_::Load =>
				Some(Box::new(load_::Item_::new(&w.kws_))),
			keyword_::Id_::Print =>
				Some(Box::new(print_::Item_::new(&w.kws_))),
			keyword_::Id_::Expl =>
				Some(Box::new(super::super::expl_::Item_::new(&w.kws_))),
			keyword_::Id_::Exec =>
				Some(Box::new(exec_::Item_::new(&w.kws_))),
			_ => None,
		}
	}

	fn add_text__(&self, text:&mut Vec<u8>, is:&mut IsText_, ret:&mut Ret_, w:&World_) -> Result2_ {
		if !text.is_empty() {
			let s = String::from_utf8(text.clone()).unwrap();
			let code:code_::BI_ =
				if is.undef__() {
					Box::new(undef_::Item_::new(&w.kws_, &s))
				} else {
					Box::new(text_::Item_::new(&w.kws_, &s))
				};
			if w.dbg_.par_lc_ {
				lc4__!("{}", s);
			}
			text.clear();
			is.clear__();
			self.add_code__(code, ret)
		} else {
			ok__()
		}
	}
	fn add_code__(&self, code:code_::BI_, ret:&mut Ret_) -> Result2_ {
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
	fn add_code2__(&self, code:impl code_::Item_ + 'static, ret:&mut Ret_) -> Result2_ {
		self.add_code__(Box::new(code), ret)
	}
}
