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
			let mut can_redir = true;
			for i in v.iter().skip(1) {
				let i:&str = i;
				match i {
					"" => pf = true,
					"错" => ep = true,
					"原" => can_redir = false,
					_ => return self.super_.err__(&[&as_ref__!(env.w).text__(i), "无效选项"].concat())
				}
			}
			if can_redir {
				let mut ret = result_::List_::new();
				let mut q = Some(env.q.clone());
				loop {
					if q.is_none() {break;}
					let q2 = q.unwrap();
					let q2 = as_ref__!(q2);
					if q2.defs_.get__(&self.kw__().s_, false, &mut ret) {
						if let result_::Val_::S(src) = &*as_ref__!(as_ref__!(ret.a_[0]).val_) {
							let q = Qv_::new2(Some(env.q.clone()));
							as_mut_ref__!(q.args_).add__(s);
							return eval_::hello__(&src, &code_::Env_::new2(t__(q), env));
						}
					}
					q = q2.up_.clone();
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
			for c in env.gd.guandao_jie_.chars() {
				match c {
					'1' =>
						as_mut_ref__!(env.ret).add__(s),
					_ => return as_ref__!(env.w).no_guandaojie__(c)
				}
			}
		}
		ok__()
	}
}
