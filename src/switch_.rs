use super::u_::*;

pub struct Item_ {
	pub super_:code_::Item1_,
	pub a_:code_::OL_,
	pub case_:code_::OL_,
	from_:Vec<(usize, usize)>,
	default_:Option<usize>,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:code_::Item1_::new(&kws.switch_), a_:None, case_:None, from_:vec![], default_:None}
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
		if a[end].kw__().id_ != keyword_::Id_::BeginBlock {
			return result2_::err2__("无体")
		}
		let a2 = a.remove(end);
		let mut case = code_::List_::new();
		let mut default = core::usize::MAX;
		let mut i2 = 0;
		let mut i3 = 0;
		let a3 = a2.a__().unwrap();
		/*if a3.len() < 4 {
			return result2_::err2__("无体")
		}*/
		let mut default__ = |idx| {
			if default != core::usize::MAX {
				return result2_::err2__("只能有一个其他")
			}
			default = idx - 1;
			//lc3__!("{:?}",default);
			ok__()
		};
		for (idx, i) in a3.iter().enumerate() {
			//lc3__!("\n{} {}{}",idx,i.s2__(),i.kw__().s_);
			match i.kw__().id_ {
				keyword_::Id_::Jvhao => {
					match i2 {
						0 => {}
						1 => {default__(idx)?;}
						_ => {
							let item = (i3, idx - 1);
							//lc5__!("{:?}",item);
							self.from_.push(item);
						}
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
				1 => {default__(i3 + 1)?;}
				_ => self.from_.push((i3, a3.len()))
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

	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
		let mut ret2 = result_::List_::new();
		t_::o__(&self.a_).hello__(gd, q.clone(), w, &mut ret2)?;
		let v = ret2.to_vec__();
		let s = if v.is_empty() {""} else {&v[0]};
		let case = t_::o__(&self.case_)[0].a__().unwrap();
		//w.dbg_.tree__(case, w);

		let mut gd2 = gd;
		gd2.jvhao_ = true;
		gd2.dunhao_ = true; 

		let mut gd3 = gd;
		gd3.jvhao_ = true;
		gd3.dunhao_ = false; 

		let mut need_default = true;
		let mut idx = 0;
		while idx < self.from_.len() {
			let (mut from, to) = self.from_[idx];
			//lc3__!("\n({}-{})",from,to);
			while from < to {
				let mut ret3 = result_::List_::new();
				case.hello2__(&mut from, to, gd2, q.clone(), w, &mut ret3)?;
				let mut s2 = String::new();
				for i in ret3.iter() {
					i.borrow().s2__(&mut s2, false, false, true)
				}
				if w.dbg_.lc_ {
					lc3__!("\n({}={})", s, s2);
				}
				if s2 == s {
					let mut from2 = to;
					//lc3__!("({})",from2);
					need_default = false;
					case.hello2__(&mut from2, core::usize::MAX, gd3, q.clone(), w, ret)?;
				}
			}
			idx += 1;
		}
		if need_default {
			if let Some(mut i) = self.default_ {
				case.hello2__(&mut i, core::usize::MAX, gd3, q.clone(), w, ret)?;
			}
		}
		ok__()
	}
}