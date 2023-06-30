pub const REM_: &str = "值";

pub fn equ_name__(s:&str, name:&str, ret:&mut super::result_::List_) -> bool {
	if s == "-" || s == "true" {
		ret.add__(name);
		return true
	}
	false
}

#[derive(Default)]
pub struct EquName_ {
	pub equ_:bool,
	pub name_:String,
}

impl EquName_ {
	pub fn by__(&mut self, s:&str) {
		self.equ_ = true;
		self.name_ = s.to_string();
	}
}
