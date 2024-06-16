use super::{*, super::*};

mod layer_;
use layer_::*;
mod add_;
use add_::*;

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

pub struct Mut2_ {
	kw_:keyword_::ORI_,
	i_:i32,
	u_:usize,
	bp_:bool,
	#[cfg(debug_assertions)]
	lc_:bool,
}
impl Mut2_ {
	fn new() -> Self {
		Self{i_:-1, kw_:None, u_:0, bp_:false,
			#[cfg(debug_assertions)] lc_:false}
	}
	fn reset__(&mut self) {
		self.i_ = -1;
		self.kw_ = None;
		self.bp_ = false;
		self.u_ = 0;
	}
	#[cfg(debug_assertions)]
	fn bp__(&self, s:&str, point:bool, w:&World_) -> bool {
		if self.bp_ && w.dbg_.parbp_.contains(s) {self.pause__(point); true} else {false}
	}
	#[cfg(debug_assertions)]
	fn pause__(&self, point:bool) {
		if point {
			#[allow(non_snake_case)] #[allow(unused_variables)] let o_H_o = true;
		}
	}
	#[cfg(debug_assertions)]
	fn lc__(&self, w:&World_) -> bool {
		w.dbg_.par_lc_ || self.lc_
	}
}

type Ret0_ = Rc_<RefCell_<i32>>;
fn new_ret_0__() -> Ret0_ {Rc_::new(RefCell_::new(1))}
fn ret_0__(i:&Ret0_) -> i32 {*as_ref__!(i)}
fn ret_02__(i:&Ret0_, i2:i32) {*as_mut_ref__!(i) = i2}

type Ret_<'a> = (Ret0_,
	       &'a mut code_::List_,
	Option<&'a mut code_::List_>,
	Option<&'a mut code_::List_>);

#[derive(Clone, Default)]
pub struct Pars_ {
	pub u11_:Option<Rc_<dyn U11_>>,
}

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
	fn hello2__(&self, a2:&[u8], i:&mut usize, layer:RL_, mut2:&mut Mut2_,
			is:&mut IsText_, kws:&keywords_::A_, ret:&mut Ret_, w:&World_) -> Result2_ {
		#[cfg(debug_assertions)]
		if mut2.lc__(w) {
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
					} else if is.yuanyang__() {
						&kws.yuanyang_
					} else if is.rem__() {
						&kws.rem_
					} else if is.rem2__() {
						&kws.rem2_
					} else if is.code__() {
						&kws.code_
					} else if is.text2__() {
						&kws.text2_
					} else if is.data__() {
						&kws.data_
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
				if let add@1..=2 = match kw.id_ {
					keyword_::Id_::BeginRem			=> is.add3__('r', '+'),
					keyword_::Id_::EndRem			=> is.add3__('r', '-'),
					keyword_::Id_::BeginText		=> is.add2__('t', '+'),
					keyword_::Id_::EndText			=> is.add2__('t', '-'),
					keyword_::Id_::BeginYuanyang	=> is.add2__('y', '+'),
					keyword_::Id_::EndYuanyang		=> is.add2__('y', '-'),
					keyword_::Id_::BeginCode		=> is.add2__('c', '+'),
					keyword_::Id_::EndCode			=> is.add2__('c', '-'),
					keyword_::Id_::BeginText2		=> is.add2__('e', '+'),
					keyword_::Id_::EndText2			=> is.add2__('e', '-'),
					_ => 0,
				} {
					if add == 2 {
						if is.undef__() {
							add_text__(&mut text, is, ret, w, mut2)?;
						};
						add_kw2__(kw, &mut text);
					}
					#[cfg(debug_assertions)]
					if mut2.lc__(w) {
						w.dbg_.lc2__(kw, &kw.s_);
					}
					continue
				}
				match kw.id_ {
					keyword_::Id_::BeginRem2 |
					keyword_::Id_::EndRem2 |
					keyword_::Id_::BeginVar |
					keyword_::Id_::EndVar => {
						if is.as2__() {
							#[cfg(debug_assertions)]
							if mut2.lc__(w) {
								w.dbg_.lc2__(kw, &kw.s_);
							}
							add_kw2__(kw, &mut text);
							continue
						}
					}
					_ => {}
				}
				add_text__(&mut text, is, ret, w, mut2)?;
				#[cfg(debug_assertions)]
				if mut2.lc__(w) || mut2.bp__("-kw-", false, w) {
					w.dbg_.lc2__(kw, &kw.s_);
				}
				if let keyword_::Id_::Maohao = kw.id_ {continue}
				if match kw.id_ {
					keyword_::Id_::Jvhao |
					keyword_::Id_::EndBlock | keyword_::Id_::BeginBlock |
					keyword_::Id_::Equ => true,
					_ => false,
				} || kw.g_.if_ || kws.if_.contains(kw)
				  || kw.g_.if2_ || kws.if2_.contains(kw) {
					if layer.i1__(kw.clone(), kws, mut2, w) {
						break 'l1
					}
				}
				match kw.id_ {
					keyword_::Id_::Equ |
					keyword_::Id_::Then => ret_02__(&ret.0, 2),
					keyword_::Id_::Else => ret_02__(&ret.0, 3),
					_ => {}
				}
				if match kw.id_ {
					keyword_::Id_::Jvhao |
					keyword_::Id_::Dunhao |
					keyword_::Id_::Brkpoint => true,
					_ => false
				} || kw.g_.if_ || kws.if_.contains(kw) {
					add_kw__(kw, ret)?;
					continue
				}
				match kw.id_ {
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
					keyword_::Id_::ParBrkpoint => {
						mut2.bp_ = true;
						#[cfg(debug_assertions)]
						if w.dbg_.parbp_.contains("-lc-") {
							mut2.lc_ = true;
						}
						continue
					}
					_ => {}
				}
				let ret2_0 = new_ret_0__();
				let mut ret2_2 = None;
				let mut ret2_3 = None;
				let mut ret2_2t = code_::List_::new();
				let mut ret2_3t = code_::List_::new();
				#[cfg(debug_assertions)]
				if mut2.lc__(w) {
					lc__!("({}", layer.i_);
				}
				match kw.id_ {
					keyword_::Id_::If => {
						ret2_2 = Some(&mut ret2_2t);
						ret2_3 = Some(&mut ret2_3t);
					}
					_ => {
						if kw.g_.set_ || kws.set_.contains(kw) {
							ret2_2 = Some(&mut ret2_2t);
						}
					}
				};
				let layer2 = Layer_::new3(layer.i_ + 1, Some(kw.clone()), ret2_0.clone(), Some(layer.clone()));
				let mut ret2_1 = code_::List_::new();
				#[cfg(debug_assertions)]
				mut2.bp__("-2-", true, w);
				self.hello2__(a2, i, Layer_::new2(layer2), mut2, is, kws, &mut (ret2_0, &mut ret2_1, ret2_2, ret2_3), w)?;
				#[cfg(debug_assertions)]
				if mut2.lc__(w) {
					lc__!("({}{} codes[{}]", ret_0__(&ret.0), kw.s_, ret2_1.len());
					if !ret2_2t.is_empty() {
						lc__!("2[{}]", ret2_2t.len());
					}
					if !ret2_3t.is_empty() {
						lc__!("3[{}]", ret2_3t.len());
					}
					lc__!(")\n");
				}
				if let Some(code2) = code2__(kw.clone(), &ret2_1, self, w) {
					{
						let mut r = as_mut_ref__!(code2);
						w.ret4__(r.add__(ret2_1), kw)?;
						r.add2__(ret2_2t)?;
						r.add3__(ret2_3t)?;
					}
					add_code__(code2, ret)?;
				} else {return result2_::err__([&w.text__(&kw.s_), "没实现"].concat())}
				if mut2.u_ > 0 {
					#[cfg(debug_assertions)]
					if mut2.lc__(w) {
						lc__!("(ceng u {})\n", *i);
					}
					*i -= mut2.u_;
					mut2.reset__();
					continue 'l1;
				}
				if mut2.i_ >= 0 && layer.i_ > mut2.i_ {
					#[cfg(debug_assertions)]
					if mut2.lc__(w) {
						lc__!("(ceng{}>{})\n", layer.i_, mut2.i_);
					}
					break 'l1;
				}
				if let Some(kw) = &mut2.kw_ {
					if match kw.id_ {
						keyword_::Id_::Jvhao => true,
						_ => false
					} || kw.g_.if_ || kws.if_.contains(kw) {
						add_kw__(&kw, ret)?;
					} else {
						match kw.id_ {
							keyword_::Id_::Then => ret_02__(&ret.0, 2),
							keyword_::Id_::Else => ret_02__(&ret.0, 3),
							_ => {}
						}
					}
				}
				mut2.reset__();
				continue
			}
			if !kw2.is_empty() {continue 'l1}
			let c = a2[*i];
			if is.add__(c) {
				if is.need_clear__() {
					add_text__(&mut text, is, ret, w, mut2)?;
				}
				text.push(c);
			}
			*i += 1
		}
		add_text__(&mut text, is, ret, w, mut2)
	}
	pub fn hello__(&self, s:&str, fit:impl Fn(&mut IsText_), codes:&mut code_::List_, w:&World_) -> Result2_ {
		let a2 = s.as_bytes();
		let mut i = 0;
		let mut is = Default::default();
		fit(&mut is);
		let kws = keywords_::A_::new(&w.kws_);
		let mut2 = &mut Mut2_::new();
		#[cfg(debug_assertions)]
		if mut2.lc__(w) {
			println!()
		}
		let ret2_0 = new_ret_0__();
		let ret = w.ret3__(self.hello2__(a2, &mut i, Layer_::new2(Layer_::new(ret2_0.clone())), mut2,
			&mut is, &kws, &mut (ret2_0, codes, None, None), w), a2, 0, i);
		#[cfg(debug_assertions)]
		if mut2.lc__(w) {
			println!()
		}
		ret
	}
}
