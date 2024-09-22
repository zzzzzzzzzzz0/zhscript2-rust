use super::super::{*, result_::*, super::{as_ref__}};
	
pub fn backarg__(self4:&mut List_, n:usize, w:&World_, ret:&mut List_) -> Result2_ {
	if self4.dunhao_.len() + 1 < n {
		return result2_::err2__("倒挂不足")
	}
	let i = if self4.dunhao_.len() < n {
		0
	} else {
		self4.dunhao_.len() - n
	};
	let i2 = if self4.dunhao_.len() < n {
		-1
	} else {
		self4.dunhao_[i] as i32
	};
	while i < self4.dunhao_.len() {
		self4.dunhao_.pop();
	}
	ret.ins__(Rc_::new(RefCell_::new(Item_ {val_:v__(Val_::K(w.kws_.dunhao_.clone())), rem_:vec![]})));
	let mut idx = self4.len() as i32;
	if idx > 0 {
		loop {
			idx -= 1;
			if idx <= i2 {
				break
			}
			let idx = idx as usize;
			if if let Val_::K(kw) = &*as_ref__!(as_ref__!(self4[idx]).val_) {
				match kw.id_ {
					keyword_::Id_::Jvhao => true,
					_ => false
				}
			} else {false} {
				//self4.rm__(idx);
				break
			}
			ret.ins__(self4.rm__(idx).clone());
		}
	}
	ok__()
}
