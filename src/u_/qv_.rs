use super::{*, equ_name_::EquName_, super::{var_::arg_::Argi_, as_ref__, as_mut_ref__}};

pub mod rem4_;

pub type T_ = super::T_<Qv_>;

pub fn val__(name:&str, val:&str, q:T_, w:world_::T_) {
	val2__(name, result_::si__(val), false, false, false, q, w)
}
pub fn val2__(name:&str, val_:result_::I_, is_alias:bool, is_priv:bool, gang_equ_name:bool, q:T_, w:world_::T_) {
	for__(q, w, |q, _, _| {
		as_mut_ref__!(q).vars_.val2__(name, val_.clone(), if is_alias {VarTyp_::Alias} else {VarTyp_::Val},
			is_priv, gang_equ_name);
		Some(())
	});
}

pub fn def__(name:&str, val:&str, argc:usize, backarg:usize, names:def_::ORL_, q:T_, w:world_::T_) -> Result2_ {
	for__(q, w, |q, w, _| {
		Some(as_mut_ref__!(q).defs_.val__(name, val, argc, backarg, names, Some(w)))
	}).unwrap()
}

pub fn get__(name:&str, is_has:bool, equ_name:&EquName_, can_up:bool, no_self:bool, env:&code_::Env_,
		ret_alias:&mut result_::List_, ret_equ_name:&mut EquName_, q_get:&mut T_) -> bool {
	let ret2 = for2__(env.q.clone(), env.w.clone(), |q, _, no_self| {
		let q2 = as_ref__!(q);
		let mut ret = as_mut_ref__!(env.ret);
		if q2.vars_.get2__(name, is_has, equ_name, no_self, &mut ret, ret_alias, ret_equ_name) {
			*q_get = q.clone();
			return Some(())
		}
		if q2.defs_.get__(name, is_has, &mut ret) {
			*q_get = q.clone();
			return Some(())
		}
		if let Some(argnames) = &q2.argnames_ {
			for i in argnames.iter() {
				if i.is_ge_ {
					continue
				}
				if name == i.s_ {
					Argi_::hello__(i.i_, is_has, i.equ_name_, name, q.clone(), &mut ret);
					*q_get = q.clone();
					return Some(())
				}
			}
		}
		None
	}, !is_has && can_up, no_self);
	ret2.is_some()
}

pub fn find_def__(cs:&[char], from:usize, end:usize, paichu:&[String], q:T_, w:world_::T_) -> Option<(usize, usize, def_::I_, T_)> {
	for idx in from..end {
		let ret = for__(q.clone(), w.clone(), |q, _, _|
			if let Some((idx3, len, def)) = as_ref__!(q).defs_.find__(cs, idx, true, true, paichu) {
				Some((idx3, len, def, q.clone()))
			} else {None}
		);
		if ret.is_some() { return ret }
	}
	None
}

pub fn for__<T>(q:T_, w:world_::T_, f:impl FnMut(T_, world_::T_, bool) -> Option<T>) -> Option<T> {
	for2__(q, w, f, true, false)
}
pub fn for2__<T>(q:T_, w:world_::T_, f:impl FnMut(T_, world_::T_, bool) -> Option<T>, next:bool, no_self:bool) -> Option<T> {
	for3__(q, w, f, next, no_self, true)
}
pub fn for3__<T>(q:T_, w:world_::T_, mut f:impl FnMut(T_, world_::T_, bool) -> Option<T>, next:bool, no_self:bool, inc_mod:bool) -> Option<T> {
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
	if inc_mod {
		for q3 in &as_ref__!(w).mods_ {
			let ret = f(q3.clone(), w.clone(), true);
			if ret.is_some() { return ret }
		}
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
	pub i_:i32,
	pub on_free_:String,
	
	pub arg0_:String,
	pub args_:super::T_<result_::List_>,
	pub argnames_:OArgNames_,
	args_to1_:bool,
	args_1_:String,
	
	pub objs_:OBJS_,
	objs_mut_:Vec<result_::RO_>,
}

impl Qv_ {
	pub fn new(name:&str) -> Self {
		Self::new3(name, None)
	}
	pub fn new2(up:Option<T_>) -> Self {
		Self::new4("", up)
	}
	pub fn new3(name:&str, up:Option<T_>) -> Self {
		Self::new6(vec![name.to_string()], "", "", None, up)
	}
	pub fn new4(src:&str, up:Option<T_>) -> Self {
		Self::new6(vec![], src, "", None, up)
	}
	pub fn new5(arg0:&str, argnames:OArgNames_, up:Option<T_>) -> Self {
		Self::new6(vec![], "", arg0, argnames, up)
	}
	pub fn new6(name_:Vec<String>, src:&str, arg0:&str, argnames_:OArgNames_, up_:Option<T_>) -> Self {
		let i_ = if let Some(up) = &up_ {
			let up = as_ref__!(up);
			up.i_ + 1
		} else {0};
		Self {up_, i_, on_free_:String::new(), vars_:var_::List_::new(), defs_:def_::List_::new(),
			name_, src_:src.to_string(), src_is_file_:false, arg0_:arg0.to_string(),
			args_:t__(result_::List_::new()), argnames_, args_to1_:false, args_1_:String::new(),
			objs_:None, objs_mut_:vec![],}
	}
	
	pub fn simp_def__(&mut self, name:&str, val:&str) -> Result2_ {
		self.defs_.val2__(name, def_::Val_::Si(val.to_string()), 0, 0, None, None)
	}
	pub fn val__(&mut self, name:&str, val:&str) {
		self.vars_.val__(name, val)
	}
	
	pub fn args_1__(&mut self) -> &String {
		if !self.args_to1_ {
			self.args_to1_ = true;
			as_mut_ref__!(self.args_).to1__(&mut self.args_1_)
		}
		&self.args_1_
	}
	pub fn args_1clear__(&mut self) {
		self.args_to1_ = false;
		self.args_1_.clear();
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
