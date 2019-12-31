pub struct Cfg_ {
	pub src_is_file_:bool,
	pub shl_:String,
}

impl Cfg_ {
	pub fn new() -> Self {
		Self {
			shl_:"".to_string(), src_is_file_:true,
		}
	}
}
