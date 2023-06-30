use super::u_::*;
#[cfg(debug_assertions)]
use super::db_c__;

pub struct Item_ {
	pub super_:code_::Item1_,
	pub a_:code_::OL_,
	pub case_:code_::OL_,
	from_:Vec<(usize, usize)>,
	default_:Option<usize>,
	one_:bool,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_, one_:bool) -> Self {
		Self {super_:code_::Item1_::new(if one_ {&kws.switch2_} else {&kws.switch_}),
			a_:None, case_:None, from_:vec![], default_:None, one_}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}

	fn add__(&mut self, mut a:code_::List_) -> Result2_ {
		let len = a.len();
		if len < 2 {
			return result2_::qve__()
		}
		let end = len - 1;
		if as_ref__!(a[end]).kw__().id_ != keyword_::Id_::BeginBlock {
			return result2_::err2__("无体")
		}
		let a2 = a.remove(end);
		let mut case = code_::List_::new();
		let mut default = core::usize::MAX;
		{
			let mut i2 = 0;
			let mut i3 = 0;
			let r_a2 = as_ref__!(a2);
			let a3 = r_a2.a__().unwrap();
			let mut defaultf = |idx| {
				if default != core::usize::MAX {
					return result2_::err2__("只能有一个其他")
				}
				default = idx - 1;
				ok__()
			};
			let mut add = |i3, idx| {
				let item = (i3, idx - 1);
				self.from_.push(item);
			};
			for (idx, i) in a3.iter().enumerate() {
				let i = as_ref__!(i);
				match i.kw__().id_ {
					keyword_::Id_::Jvhao /*| keyword_::Id_::Douhao*/ => {
						match i2 {
							0 => {}
							1 => {defaultf(idx)?;}
							_ => add(i3, idx)
						}
						i2 = 0;
						i3 = idx + 1;
					}
					_ => {
						i2 += 1;
					}
				}
			}
			if i3 < a3.len() {
				match i2 {
					1 => {defaultf(i3 + 1)?;}
					_ => add(i3, a3.len())
				}
			}
		}
		case.push(a2);
		self.case_ = Some(case);
		if default != core::usize::MAX {
			self.default_ = Some(default);
		}
		self.a_ = Some(a);
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {t_::some__(&&self.a_)}
	fn a2__(&self) -> code_::ORL_ {t_::some__(&self.case_)}

	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		let ret2 = t__(result_::List_::new());
		t_::o__(&self.a_).hello__(&code_::Env_::new6(ret2.clone(), env))?;
		let v = as_ref__!(ret2).to_vec__();
		let s = if v.is_empty() {""} else {&v[0]};
		let case = as_ref__!(t_::o__(&self.case_)[0]);
		let case = case.a__().unwrap();

		let gd3 = code_::Opt_ {jvhao_:true, dunhao_:false, ..env.gd};

		let mut case2 = vec![];
		let mut idx = 0;
		while idx < self.from_.len() {
			let (mut from, to) = self.from_[idx];

			let ret3 = t__(result_::List_::new());
			case.hello2__(&mut from, to, &code_::Env_::new7(gd3, ret3.clone(), env))?;
			let mut s2 = String::new();
			let casehe = |s2:&mut String, case2:&mut Vec<(usize, usize)>| {
				if s2 == s {
					let i = (to, if idx + 1 == self.from_.len() {
						core::usize::MAX
					} else {
						self.from_[idx + 1].0
					});
					#[cfg(debug_assertions)]
					if db_c__!("-if-", env) {
						lc3__!("{:?}", i);
					}
					case2.push(i);
				}
				s2.clear();
			};
			let end = as_ref__!(ret3).len();
			if end > 0 {
				let end = end - 1;
				for (idx3, i) in as_ref__!(ret3).iter().enumerate() {
					let dunhao = as_ref__!(i).dunhao__();
					if !dunhao {
						as_ref__!(i).s_inc_some_kw__(&mut s2);
					}
					if dunhao || idx3 == end {
						#[cfg(debug_assertions)]
						if db_c__!("-if-", env) {
							lc3__!("\n({} {}={})", idx, s, s2);
						}
						casehe(&mut s2, &mut case2);
					}
				}
			} else {
				casehe(&mut s2, &mut case2);
			}

			idx += 1;
		}
		if case2.is_empty() {
			if let Some(mut i) = self.default_ {
				case.hello2__(&mut i, core::usize::MAX, &code_::Env_::new3(gd3, env))?;
			}
		} else {
			if self.one_ && case2.len() > 1 {
				return result2_::err2__("非唯一");
			}
			for (mut from2, to2) in case2 {
				case.hello2__(&mut from2, to2, &code_::Env_::new3(gd3, env))?;
			}
		}
		ok__()
	}
}