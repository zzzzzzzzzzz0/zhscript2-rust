use super::{*, super::{as_ref__, as_mut_ref__}};

pub type T_ = Rc_<RefCell_<Qv_>>;

pub fn t__(q:Qv_) -> T_ {
	Rc_::new(RefCell_::new(q))
}

pub fn val__(name:&str, val:&str, q:T_, w:world_::T_) {
	val2__(name, result_::si__(val), false, q, w)
}
pub fn val2__(name:&str, val_:result_::I_, is_alias:bool, q:T_, w:world_::T_) {
	for__(q, w, |q, _| {
		as_mut_ref__!(q).vars_.val2__(name, val_.clone(), if is_alias {VarTyp_::Alias} else {VarTyp_::Val});
		Some(())
	});
}

pub fn def__(name:&str, val:&str, argc:usize, names:def_::ORL_, q:T_, w:world_::T_) -> Result2_ {
	for__(q, w, |q, w| {
		Some(as_mut_ref__!(q).defs_.val__(name, val, argc, names, Some(w)))
	}).unwrap()
}

pub fn get__(name:&str, is_has:bool, can_up:bool, no_self:bool, q:T_, w:world_::T_,
		ret:&mut result_::List_, ret_alias:&mut result_::List_, q_get:&mut T_) -> bool {
	let ret2 = for2__(q, w, |q, _| {
		let q2 = as_ref__!(q);
		if q2.vars_.get__(name, is_has, ret, ret_alias) {
			*q_get = q.clone();
			return Some(())
		}
		if q2.defs_.get__(name, is_has, ret) {
			*q_get = q.clone();
			return Some(())
		}
		None
	}, !is_has && can_up, no_self);
	is_has || ret2.is_some()
}

pub fn find_def__(cs:&[char], from:usize, end:usize, paichu:&Vec<String>, q:T_, w:world_::T_) -> Option<(usize, usize, def_::I_)> {
	for idx in from..end {
		let ret = for__(q.clone(), w.clone(), |q, _|
			as_ref__!(q).defs_.find__(cs, idx, paichu)
		);
		if ret.is_some() { return ret }
	}
	None
}

pub fn for__<T>(q:T_, w:world_::T_, f:impl FnMut(T_, world_::T_) -> Option<T>) -> Option<T> {
	for2__(q, w, f, true, false)
}
pub fn for2__<T>(q:T_, w:world_::T_, mut f:impl FnMut(T_, world_::T_) -> Option<T>, next:bool, no_self:bool) -> Option<T> {
	let mut no_self = no_self;
	let mut q2 = Some(q);
	while let Some(q3) = &q2 {
		if no_self {
			no_self = false
		} else {
			let ret = f(q3.clone(), w.clone());
			if ret.is_some() {return ret}
		}
		if !next {return None}
		let q5 = 
			if let Some(q4) = &as_ref__!(q3).up_ {
				Some(q4.clone())
			} else {
				break
			};
		q2 = q5;
	}
	for q3 in &as_ref__!(w).mods_ {
		let ret = f(q3.clone(), w.clone());
		if ret.is_some() { return ret }
	}
	None
}

#[derive(Clone)]
pub struct Qv_ {
	pub name_:Vec<String>,
	pub src_:String,
	pub vars_:var_::List_,
	pub defs_:def_::List_,
	pub up_:Option<T_>,
	
	pub args_:result_::List_,
	pub args2_:Vec<String>,
	pub argnames_:OArgNames_,
	args_to1_:bool,
	args_1_:String,
	
	pub objs_:def_::OBJS_,
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
	pub fn new5(src:&str, argnames_:OArgNames_, up_:std::option::Option<T_>) -> Self {
		Self {up_, vars_:var_::List_::new(), defs_:def_::List_::new(),
			name_:vec![], src_:src.to_string(),
			args_:result_::List_::new(), args2_:vec![], argnames_,
			args_to1_:false, args_1_:String::new(),
			objs_:None}
	}
	
	pub fn simp_def__(&mut self, name:&str, val:&str) -> Result2_ {
		self.defs_.val2__(name, def_::Val_::Si(val.to_string()), 0, None, None)
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
	
	#[allow(dead_code)]	
	pub fn objs__<T>(&self, i:usize) -> Option<&T> {
		if let Some(objs) = &self.objs_ {
			let objs = as_ref__!(objs);
			if i < objs.len() {
				unsafe {
					return std::mem::transmute::<def_::ObjTyp_, *const T>(objs[i].p_).as_ref()
				}
			}
		}
		None
	}
}