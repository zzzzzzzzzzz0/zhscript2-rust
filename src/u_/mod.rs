mod cfg_;
pub mod world_;
pub mod pars_;
mod is_text_;
pub mod result_;
pub mod result2_;
pub mod code_;
pub mod jump_;
pub mod keyword_;
mod keywords_;
pub mod codes_cache_;
pub mod qv_;
mod var_;
pub mod def_;
pub mod eval_;
pub mod expl_;
pub mod equ_name_;

//#[cfg(debug_assertions)]
#[macro_use]
pub mod dbg_;
pub mod t_;

use super::cfg_if;

type Cfg_ = cfg_::Cfg_;
//#[cfg(debug_assertions)]
pub type Dbg_ = dbg_::Dbg_;
pub type World_ = world_::World_;
pub type Qv_ = qv_::Qv_;
pub type VarTyp_ = var_::Typ_;
pub type IsText_ = is_text_::IsText_;

pub type OArgNames_ = def_::ORAN_;

pub type Result2_ = result2_::Item_;
pub fn ok__() -> Result2_ {Ok(())}

cfg_if! {
	if #[cfg(feature = "thread")] {
		pub type Rc_<T> = std::sync::Arc<T>;
		pub type RefCell_<T> = std::sync::RwLock<T>;
	} else {
		pub type Rc_<T> = std::rc::Rc<T>;
		pub type RefCell_<T> = std::cell::RefCell<T>;
	}
}

pub type T_<T> = Rc_<RefCell_<T>>;
pub fn t__<T>(q:T) -> T_<T> {
	Rc_::new(RefCell_::new(q))
}

#[cfg(not(feature = "thread"))]
#[macro_use]
mod thread_ {
	#[macro_export]
	macro_rules! as_ref__ {($v:expr) => ($v.borrow())}
	#[macro_export]
	macro_rules! as_mut_ref__ {($v:expr) => ($v.borrow_mut())}
}

#[cfg(feature = "thread")]
#[macro_use]
mod thread_ {
	#[macro_export]
	macro_rules! as_ref__ {($v:expr) => ($v.read().unwrap())}
	#[macro_export]
	macro_rules! as_mut_ref__ {($v:expr) => ($v.write().unwrap())}
}
