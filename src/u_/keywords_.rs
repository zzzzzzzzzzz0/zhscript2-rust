use super::*;

pub struct A_ {
	pub rem_:Vec<keyword_::RI_>,
	pub rem2_:Vec<keyword_::RI_>,
	pub var_:Vec<keyword_::RI_>,
	pub text_:Vec<keyword_::RI_>,
	pub text2_:Vec<keyword_::RI_>,
	pub yuanyang_:Vec<keyword_::RI_>,
	pub code_:Vec<keyword_::RI_>,
	
	pub print_:Vec<keyword_::RI_>,
	pub if_:Vec<keyword_::RI_>,
	pub if2_:Vec<keyword_::RI_>,
	pub set_:Vec<keyword_::RI_>,
}

impl A_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		let mut rem_ = vec![];
		let mut rem2_ = vec![];
		let mut var_ = vec![];
		let mut text_ = vec![];
		let mut text2_ = vec![];
		let mut yuanyang_ = vec![];
		let mut code_ = vec![];
		
		let mut print_ = vec![];
		let mut if_ = vec![];
		let mut if2_ = vec![];
		let mut set_ = vec![];
		
		for i in &kws.a_ {
			match i.id_ {
				keyword_::Id_::BeginRem |
				keyword_::Id_::EndRem => {
					rem_.push(i.clone());
					code_.push(i.clone());
					text2_.push(i.clone());
				}
				keyword_::Id_::BeginRem2 |
				keyword_::Id_::EndRem2 => {
					rem2_.push(i.clone());
					var_.push(i.clone());
				}
				keyword_::Id_::BeginVar |
				keyword_::Id_::EndVar => {
					rem2_.push(i.clone());
					var_.push(i.clone());
					text_.push(i.clone());
					text2_.push(i.clone());
				}
				keyword_::Id_::Brkpoint |
				keyword_::Id_::ParBrkpoint => {
					var_.push(i.clone());
				}
				keyword_::Id_::BeginText |
				keyword_::Id_::EndText => {
					text_.push(i.clone());
					code_.push(i.clone());
					text2_.push(i.clone());
				}
				keyword_::Id_::BeginText2 |
				keyword_::Id_::EndText2 => {
					text2_.push(i.clone());
					code_.push(i.clone());
				}
				keyword_::Id_::BeginYuanyang |
				keyword_::Id_::EndYuanyang => {
					yuanyang_.push(i.clone());
					code_.push(i.clone());
					text2_.push(i.clone());
				}
				keyword_::Id_::BeginCode |
				keyword_::Id_::EndCode => {
					code_.push(i.clone());
					text2_.push(i.clone());
				}
				_ => {}
			}
			match i.id_ {
				keyword_::Id_::Return |
				keyword_::Id_::Quit |
				keyword_::Id_::Mod |
				keyword_::Id_::Name |
				keyword_::Id_::Eval |
				keyword_::Id_::Load |
				keyword_::Id_::Print |
				keyword_::Id_::Exec |
				keyword_::Id_::Set |
				keyword_::Id_::Alias |
				keyword_::Id_::Def => {
					print_.push(i.clone());
				}
				keyword_::Id_::Dengyu |
				keyword_::Id_::Xiaoyudengyu |
				keyword_::Id_::Xiaoyu |
				keyword_::Id_::Dayudengyu |
				keyword_::Id_::Dayu |
				keyword_::Id_::Not |
				keyword_::Id_::And |
				keyword_::Id_::Or => {
					if_.push(i.clone());
				}
				keyword_::Id_::Then |
				keyword_::Id_::Else => {
					if2_.push(i.clone());
				}
				_ => {}
			}
			match i.id_ {
				keyword_::Id_::Set |
				keyword_::Id_::Alias |
				keyword_::Id_::Def => {
					set_.push(i.clone());
				}
				_ => {}
			}
		}
		Self {rem_, rem2_, var_, text_, yuanyang_, code_, text2_, print_, if_, if2_, set_}
	}
}
