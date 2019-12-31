use super::*;
use std::rc::Rc;
use std::cell::RefCell;

pub type T_ = Rc<RefCell<Qv_>>;

pub fn t__(q:Qv_) -> T_ {
	Rc::new(RefCell::new(q))
}
	
pub fn val__(name:&str, val:&str, q:qv_::T_, w:&mut World_) {
	val2__(name, val, false, q, w)
}
pub fn val2__(name:&str, val:&str, is_alias:bool, q:qv_::T_, w:&mut World_) {
	for__(q, w, |q, _| {
		q.borrow_mut().vars_.val2__(name, val, if is_alias {VarTyp_::Alias} else {VarTyp_::Val});
		Some(())
	});
}

pub fn def__(name:&str, val:&str, argc:usize, names:def_::ORL_, q:qv_::T_, w:&mut World_) -> Result2_ {
	for__(q, w, |q, w| {
		Some(q.borrow_mut().vars_.def__(name, val, argc, names, Some(w)))
	}).unwrap()
}

pub fn get__(name:&str, is_has:bool, q:qv_::T_, w:&mut World_, ret:&mut result_::List_) -> Result2_ {
	let ret2 = for2__(q, w, |q, _| {
		if q.borrow().vars_.get__(name, is_has, ret) {Some(())} else {None}
	}, !is_has);
	if ret2.is_none() && !is_has {
		result2_::err__(["变量", &w.text__(name), "不存在"].concat())
	} else {
		ok__()
	}
}

pub fn find_def__(cs:&[char], from:usize, end:usize, paichu:&Vec<String>, q:qv_::T_, w:&mut World_) -> Option<(usize, usize, var_::RRI_)> {
	for__(q, w, |q, _| q.borrow_mut().vars_.find_def__(cs, from, end, paichu))
}

pub fn for__<T>(q:qv_::T_, w:&mut World_, f:impl FnMut(qv_::T_, &World_) -> Option<T>) -> Option<T> {
	for2__(q, w, f, true)
}
pub fn for2__<T>(q:qv_::T_, w:&mut World_, mut f:impl FnMut(qv_::T_, &World_) -> Option<T>, next:bool) -> Option<T> {
	let mut q2 = Some(q);
	while let Some(q3) = &q2 {
		let ret = f(q3.clone(), w);
		if ret.is_some() {return ret}
		if !next {return None}
		let q5 = 
			if let Some(q4) = &q3.borrow_mut().up_ {
				Some(q4.clone())
			} else {
				break
			};
		q2 = q5;
	}
	for q3 in &w.mods_ {
		let ret = f(q3.clone(), w);
		if ret.is_some() { return ret }
	}
	None
}

pub struct Qv_ {
	pub name_:String,
	pub src_:String,
	vars_:var_::List_,
	pub up_:Option<T_>,
	
	pub args_:result_::List_,
	pub args2_:Vec<String>,
	pub argnames_:OArgNames_,
	args_to1_:bool,
	args_1_:String,
}

impl Qv_ {
	pub fn new() -> Self {
		Self::new2(None)
	}
	pub fn new2(up:Option<T_>) -> Self {
		Self::new4("", up)
	}
	pub fn new4(src:&str, up:std::option::Option<T_>) -> Self {
		Self::new5(src, None, up)
	}
	pub fn new5(src:&str, argnames:OArgNames_, up:std::option::Option<T_>) -> Self {
		Self {up_:up, vars_:var_::List_::new(),
			name_:"".to_string(), src_:src.to_string(),
			args_:result_::List_::new(), args2_:vec![], argnames_:argnames,
			args_to1_:false, args_1_:String::new()}
	}
	
	pub fn simp_def__(&mut self, name:&str, val:&str) {
		self.vars_.def2__(name, val, true, 0, None, None).unwrap();
	}
	pub fn val__(&mut self, name:&str, val:&str) {
		self.vars_.val__(name, val)
	}
	
	pub fn args_1__(&mut self) -> &String {
		if !self.args_to1_ {
			self.args_to1_ = true;
			self.args_.to1__(&mut self.args_1_)
		}
		&self.args_1_
	}
}