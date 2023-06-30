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
mod return_;
mod quit_;
mod if_;
mod load_;
mod guandaodu_;
mod guandaojie_;
mod guandaojie2_;
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
	std::process::exit(u_::World_::new().hello__(&mut std::env::args()))
}
