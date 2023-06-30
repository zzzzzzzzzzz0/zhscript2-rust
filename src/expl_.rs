use super::u_::*;
#[cfg(debug_assertions)]
use super::db_c__;

pub struct Item_ {
	super_:code_::Item1_,
	a_:code_::OL_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:code_::Item1_::new(&kws.expl_), a_:None}
	}
	
	fn add_n__(&self, buf:&mut String, v:&mut Vec<expl_::Item_>) -> Result2_ {
		if !buf.is_empty() {
			if let Some(f) = t_::s2n__(&buf) {
				v.push(expl_::Item_::N(f))
			} else {
				return result2_::err__([&buf, "非数字"].concat())
			}
			buf.clear()
		}
		ok__()
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn add__(&mut self, a:code_::List_) -> Result2_ {
		if a.is_empty() {
			return result2_::qve__();
		}
		self.a_ = Some(a);
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		let ret2 = t__(result_::List_::new());
		t_::o__(&self.a_).hello__(&code_::Env_::new6(ret2.clone(), env))?;
		let v = as_ref__!(ret2).to_vec__();
		if v.is_empty() {
			return result2_::qve__();
		}

		let src = &v[0];
		let w2 = as_ref__!(env.w).clone();
		/*let mut w = as_mut_ref__!(env.w);
		let mut codes = w.codes_cache2_.get__(src);
		if codes.is_none() {*///
			let mut v2 = vec![];
			let mut buf = String::new();
			for i in src.chars() {
				match i {
					'+' | '-' | '*' | '/' | '%' | '^' | '(' | ')' => {
						self.add_n__(&mut buf, &mut v2)?;
						v2.push(expl_::Item_::C(i));
					}
					i if i >= '0' && i <= '9' || i == '.' =>
						buf.push(i),
					_ => {
						buf.push(i);
						return result2_::err__([&w2.text__(&buf), "表达式非法"].concat())
					}
				}
			}
			self.add_n__(&mut buf, &mut v2)?;

			#[cfg(debug_assertions)]
			if db_c__!("-expl-", env) {
				for i in &v2 {
					lc3__!("\n{:?}", i);
				}
			}
			/*w.codes_cache2_.set__(src, expl_::List_ {a_:v2});
			codes = w.codes_cache2_.get__(src);*/
			let codes = expl_::List_ {a_:v2};//
		//}
		let mut i = 0;
		let ret2 = codes/*.unwrap()*/.z2__(&mut i);
		#[cfg(debug_assertions)]
		if db_c__!("-expl-", env) {
			lc3__!("\n{:?}\n", ret2);
		}
		match ret2 {
			Ok((n, _)) => {
				if n.is_infinite() {
					return result2_::err2__("溢出");
				}
				if v.len() > 1 {
					let s2 = &v[1];
					if let Some(i) = t_::s2n__(s2) {
						as_mut_ref__!(env.ret).add__(format!("{:.i$}", n, i = i));
					} else {
						return result2_::err__([&w2.text__(&s2), "点后位数非法"].concat())
					}
				} else {
					as_mut_ref__!(env.ret).add__(format!("{}", n));
				}
				ok__()
			}
			Err(s) => result2_::err2__(s)
		}
	}
}
