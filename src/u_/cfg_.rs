#[derive(Clone, Debug)]
pub struct Cfg_ {
	pub src_is_file_:bool,
	pub shl_:String,
}

impl Default for Cfg_ {
	fn default() -> Self {
		Self {src_is_file_:true, shl_:String::new()}
	}
}
