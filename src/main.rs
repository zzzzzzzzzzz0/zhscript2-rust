mod u2_;
#[macro_use]
mod u_;
mod print_;
mod undef_;
mod text_;
mod eval_;
mod set_;
mod var_;
mod def_;
mod rem2_;
mod exec_;
mod block_;
mod for_;
mod break_;
mod continue_;
mod range_;
mod break2_;
mod continue2_;
mod return_;
mod quit_;
mod if_;
mod load_;
mod guandaodu_;
mod guandaojie_;
mod name_;
mod mod_;
mod switch_;
mod expl_;
mod has_;
mod alias_;
#[macro_use]
mod u3_;
mod mod_free_;

fn main() {
	let mut w = u_::World_::new();
	w.kws_.add__("为", u_::keyword_::Id_::Dunhao);
	std::process::exit(w.hello__(&mut std::env::args()))
}
