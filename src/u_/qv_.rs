use super::{*, super::{as_ref__, as_mut_ref__}};

pub type T_ = Rc_<RefCell_<Qv_>>;

pub fn t__(q:Qv_) -> T_ {
	Rc_::new(RefCell_::new(q))
}

pub fn val__(name:&str, val:&str, q:T_, w:world_::T_) {
	val2__(name, result_::si__(val), false, false, q, w)
}
pub fn val2__(name:&str, val_:result_::I_, is_alias:bool, is_priv:bool, q:T_, w:world_::T_) {
	for__(q, w, |q, _, _| {
		as_mut_ref__!(q).vars_.val2__(name, val_.clone(), if is_alias {VarTyp_::Alias} else {VarTyp_::Val}, is_priv);
		Some(())
	});
}

pub fn def__(name:&str, val:&str, argc:usize, names:def_::ORL_, q:T_, w:world_::T_) -> Result2_ {
	for__(q, w, |q, w, _| {
		Some(as_mut_ref__!(q).defs_.val__(name, val, argc, names, Some(w)))
	}).unwrap()
}

pub fn get__(name:&str, is_has:bool, can_up:bool, no_self:bool, q:T_, w:world_::T_,
		ret:&mut result_::List_, ret_alias:&mut result_::List_, q_get:&mut T_) -> Option<bool> {
	let ret2 = for2__(q, w, |q, _, no_self| {
		let q2 = as_ref__!(q);
		if q2.vars_.get__(name, is_has, no_self, ret, ret_alias) {
			*q_get = q.clone();
			return Some(true)
		}
		if q2.defs_.get__(name, is_has, ret) {
			*q_get = q.clone();
			return Some(true)
		}
		if let Some(argnames) = &q2.argnames_ {
			for i in argnames.iter() {
				if i.is_ge_ {
					continue
				}
				if name == i.s_ {
					if i.i_ < q2.args_.len__() {
						if is_has {
							ret.add__("1")
						} else {
							q2.args_.ret__(i.i_, ret);
						}
					}
					*q_get = q.clone();
					return Some(true)
				}
			}
		}
		None
	}, !is_has && can_up, no_self);
	if ret2.is_none() && is_has {
		Some(false)
	} else {
		ret2
	}
}

pub fn find_def__(cs:&[char], from:usize, end:usize, paichu:&[String], q:T_, w:world_::T_) -> Option<(usize, usize, def_::I_)> {
	for idx in from..end {
		let ret = for__(q.clone(), w.clone(), |q, _, _|
			as_ref__!(q).defs_.find__(cs, idx, true, true, paichu)
		);
		if ret.is_some() { return ret }
	}
	None
}

pub fn for__<T>(q:T_, w:world_::T_, f:impl FnMut(T_, world_::T_, bool) -> Option<T>) -> Option<T> {
	for2__(q, w, f, true, false)
}
pub fn for2__<T>(q:T_, w:world_::T_, mut f:impl FnMut(T_, world_::T_, bool) -> Option<T>, next:bool, no_self:bool) -> Option<T> {
	let mut i2 = 0;
	let mut q2 = Some(q);
	while let Some(q3) = &q2 {
		if i2 == 0 && no_self {
		} else {
			let ret = f(q3.clone(), w.clone(), i2 > 0);
			if ret.is_some() {return ret}
		}
		if !next {return None}
		i2 += 1;
		let q5 = 
			if let Some(q4) = &as_ref__!(q3).up_ {
				Some(q4.clone())
			} else {
				break
			};
		q2 = q5;
	}
	for q3 in &as_ref__!(w).mods_ {
		let ret = f(q3.clone(), w.clone(), true);
		if ret.is_some() { return ret }
	}
	None
}

//*mut u8
//*mut ::std::os::raw::c_void
pub type ObjTyp_ = *const std::ffi::c_void;
pub struct Obj_ {
	pub p_: ObjTyp_,
}
cfg_if! {
	if #[cfg(feature = "thread")] {
		unsafe impl Send for Obj_ {}
		unsafe impl Sync for Obj_ {}
	}
}
pub type BJS_ = Rc_<RefCell_<Vec<Obj_>>>;
pub type OBJS_ = Option<BJS_>;

pub fn obj__<T>(i:&T) -> Obj_ {
	unsafe {
		Obj_ {p_:std::mem::transmute::<*const T, ObjTyp_>(i)}
	}
}
pub fn objs_add__<T>(objs:&BJS_, i:&T) {
	as_mut_ref__!(objs).push(obj__(i));
}

#[derive(Clone)]
pub struct Qv_ {
	pub name_:Vec<String>,
	pub src_:String,
	pub src_is_file_:bool,
	pub vars_:var_::List_,
	pub defs_:def_::List_,
	pub up_:Option<T_>,
	pub on_free_:String,
	
	pub args_:result_::List_,
	pub argnames_:OArgNames_,
	args_to1_:bool,
	args_1_:String,
	
	pub objs_:OBJS_,
	objs_mut_:Vec<result_::RO_>,
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
		Self {up_, on_free_:String::new(), vars_:var_::List_::new(), defs_:def_::List_::new(),
			name_:vec![], src_:src.to_string(), src_is_file_:false,
			args_:result_::List_::new(), argnames_, args_to1_:false, args_1_:String::new(),
			objs_:None, objs_mut_:vec![],}
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
	pub fn obj__<T>(&self, i:usize) -> Option<&T> {
		if let Some(objs) = &self.objs_ {
			let objs = as_ref__!(objs);
			if i < objs.len() {
				unsafe {
					return std::mem::transmute::<ObjTyp_, *const T>(objs[i].p_).as_ref()
				}
			}
		}
		None
	}

	#[allow(dead_code)]
	pub fn add_obj_mut__(&mut self, i:result_::BO_) {
		self.objs_mut_.push(Rc_::new(RefCell_::new(i)))
	}
	#[allow(dead_code)]
	pub fn obj_mut__(&mut self, i:usize) -> Option<result_::RO_> {
		if i < self.objs_mut_.len() {
			return Some(self.objs_mut_[i].clone());
		}
		None
	}
}