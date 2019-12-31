mod cfg_;
mod world_;
mod pars_;
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
mod def_;
pub mod eval_;
pub mod expl_;

#[macro_use]
pub mod dbg_;
pub mod t_;

type Cfg_ = cfg_::Cfg_;
pub type World_ = world_::World_;
pub type Qv_ = qv_::Qv_;
pub type VarTyp_ = var_::Typ_;

pub type ArgNames_ = def_::ArgNames_;
pub type OArgNames_ = def_::ORAN_;

pub type Result2_ = result2_::Item_;
pub fn ok__() -> Result2_ {Ok(())}