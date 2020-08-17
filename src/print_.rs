use super::u_::*;

pub struct Item_ {
	super_:code_::Item1_,
	a_:code_::OL_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:code_::Item1_::new(&kws.print_), a_:None}
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
		if !v.is_empty() {
			let s = &v[0];
			let mut pf = false;
			let mut ep = false;
			for i in v.iter().skip(1) {
				let i:&str = i;
				match i {
					"" => pf = true,
					"错" => ep = true,
					_ => return self.super_.err__(&[&as_ref__!(env.w).text__(i), "无效选项"].concat())
				}
			}
			if ep {
				eprint!("{}", s);
			} else {
				print!("{}", s);
			}
			if pf {
				t_::pf__();
			}
			if env.gd.guandao_jie_ {
				as_mut_ref__!(env.ret).add__(s);
			}
		}
		ok__()
	}
}
