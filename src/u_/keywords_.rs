use super::*;

pub struct A_ {
	pub rem_:Vec<keyword_::RI_>,
	pub rem2_:Vec<keyword_::RI_>,
	pub var_:Vec<keyword_::RI_>,
	pub text_:Vec<keyword_::RI_>,
	pub yuanyang_:Vec<keyword_::RI_>,
	pub code_:Vec<keyword_::RI_>,
}

impl A_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {
			rem_:vec![
				kws.begin_rem_.clone(),
				kws.end_rem_.clone(),
			],
			rem2_:vec![
				kws.begin_rem2_.clone(),
				kws.end_rem2_.clone(),
				kws.begin_var_.clone(),
				kws.end_var_.clone(),
			],
			var_:vec![
				kws.begin_var_.clone(),
				kws.end_var_.clone(),
				kws.begin_rem2_.clone(),
				kws.end_rem2_.clone(),
				kws.brkpoint_.clone(),
				kws.par_brkpoint_.clone(),
			],
			text_:vec![
				kws.begin_text_.clone(),
				kws.end_text_.clone(),
				kws.begin_var_.clone(),
				kws.end_var_.clone(),
			],
			yuanyang_:vec![
				kws.begin_yuanyang_.clone(),
				kws.end_yuanyang_.clone(),
			],
			code_:vec![
				kws.begin_code_.clone(),
				kws.end_code_.clone(),
				kws.begin_text_.clone(),
				kws.end_text_.clone(),
				kws.begin_yuanyang_.clone(),
				kws.end_yuanyang_.clone(),
				kws.begin_text2_.clone(),
				kws.end_text2_.clone(),
				kws.begin_rem_.clone(),
				kws.end_rem_.clone(),
			],
		}
	}
}
