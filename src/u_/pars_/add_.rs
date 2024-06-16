use super::{*, super::super::{var_, def_, eval_, expl_}};

pub fn add_kw2__(kw:&keyword_::Item_, text:&mut Vec<u8>) {
	for &i in kw.s_.as_bytes() {
		text.push(i);
	}
}

pub fn add_kw__(kw:&keyword_::RI_, ret:&mut Ret_) -> Result2_ {
	add_code__(
		match kw.id_ {
			keyword_::Id_::Dengyu |
			keyword_::Id_::Xiaoyudengyu |
			keyword_::Id_::Xiaoyu |
			keyword_::Id_::Dayudengyu |
			keyword_::Id_::Dayu |
			keyword_::Id_::Not |
			keyword_::Id_::And |
			keyword_::Id_::Or
				=> code_::i__(code_::Item2_::new(kw)),
			keyword_::Id_::Jvhao |
			keyword_::Id_::Fenhao |
			keyword_::Id_::Douhao |
			keyword_::Id_::Dunhao |
			keyword_::Id_::Brkpoint
				=> code_::i__(code_::Item1_::new(kw)),
			_ => panic!("{} {:?}", kw.s_, kw.id_),
		}
	, ret)
}

pub fn code2__(kw:keyword_::RI_, codes:&code_::List_, self1:&Pars_, w:&World_) -> code_::OI_ {
	match kw.id_ {
		keyword_::Id_::BeginRem2 =>
			rem2_::new(&w.kws_, codes),
		keyword_::Id_::BeginVar =>
			var_::new(&w.kws_, codes),
		keyword_::Id_::BeginBlock =>
			code_::oi__(block_::Item_::new(&w.kws_)),
		keyword_::Id_::For =>
			code_::oi__(for_::Item_::new(&w.kws_, jump_::BREAK_)),
		keyword_::Id_::Break =>
			code_::oi__(break_::Item_::new(&w.kws_, jump_::BREAK_)),
		keyword_::Id_::Continue =>
			code_::oi__(continue_::Item_::new(&w.kws_, jump_::CONTINUE_)),
		keyword_::Id_::Range =>
			code_::oi__(for_::Item_::new(&w.kws_, jump_::BREAK2_)),
		keyword_::Id_::Break2 =>
			code_::oi__(break_::Item_::new(&w.kws_, jump_::BREAK2_)),
		keyword_::Id_::Continue2 =>
			code_::oi__(continue_::Item_::new(&w.kws_, jump_::CONTINUE2_)),
		keyword_::Id_::Return =>
			code_::oi__(return_::Item_::new(&w.kws_)),
		keyword_::Id_::Quit =>
			code_::oi__(quit_::Item_::new(&w.kws_)),
		keyword_::Id_::If =>
			code_::oi__(if_::Item_::new(&w.kws_)),
		keyword_::Id_::Switch =>
			code_::oi__(switch_::Item_::new(&w.kws_, false)),
		keyword_::Id_::Switch2 =>
			code_::oi__(switch_::Item_::new(&w.kws_, true)),
		keyword_::Id_::Guandaodu =>
			code_::oi__(guandaodu_::Item_::new(&w.kws_)),
		keyword_::Id_::Guandaojie =>
			code_::oi__(guandaojie_::Item_::new(&w.kws_)),
		keyword_::Id_::Guandaojie2 =>
			code_::oi__(guandaojie2_::Item_::new(&w.kws_)),
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
			code_::oi__(def_::Item_::new(&w.kws_)),
		keyword_::Id_::Has =>
			code_::oi__(has_::Item_::new(&w.kws_)),
		keyword_::Id_::Eval =>
			code_::oi__(eval_::Item_::new(&w.kws_)),
		keyword_::Id_::Load =>
			code_::oi__(load_::Item_::new(&w.kws_)),
		keyword_::Id_::Print =>
			code_::oi__(print_::Item_::new(&w.kws_)),
		keyword_::Id_::Expl =>
			code_::oi__(expl_::Item_::new(&w.kws_)),
		keyword_::Id_::Exec =>
			code_::oi__(exec_::Item_::new(&w.kws_)),
		keyword_::Id_::U11 => {
			if let Some(u11) = &self1.u11_ {
				u11.u11__(kw, &w.kws_, codes)
			} else {
				None
			}
		}
		_ => None,
	}
}

pub fn add_text__(text:&mut Vec<u8>, is:&mut IsText_, ret:&mut Ret_, w:&World_, mut2:&mut Mut2_) -> Result2_ {
	if !text.is_empty() {
		match String::from_utf8(text.clone()) {
			Ok(s) => {
				let code:code_::I_ =
				if is.undef__() {
					code_::i__(undef_::Item_::new(&w.kws_, s.clone()))
				} else {
					code_::i__(text_::Item_::new(&w.kws_, s.clone()))
				};
				#[cfg(debug_assertions)]
				if mut2.lc__(w) {
					lc4__!("{}", s);
				}
				text.clear();
				is.clear__();
				add_code__(code, ret)
			}
			Err(e) => result2_::err__(e.to_string())
		}
	} else {
		is.clear__();
		ok__()
	}
}
pub fn add_code__(code:code_::I_, ret:&mut Ret_) -> Result2_ {
	match ret_0__(&ret.0) {
		2 => {
			if let Some(ls) = &mut ret.2 {
				ls.add__(code)
			} else {
				return result2_::err2__("缺少前句")
			}
		},
		3 => {
			if let Some(ls) = &mut ret.3 {
				ls.add__(code)
			} else {
				return result2_::err2__("缺少前句")
			}
		},
		_ => ret.1.add__(code),
	}
	ok__()
}
