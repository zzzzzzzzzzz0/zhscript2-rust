// 67 14,8,13

#![allow(dead_code)]

use super::Rc_;

pub type RI_ = Rc_<Item_>;
pub type ORI_ = Option<RI_>;

#[derive(PartialEq, Debug)]
pub enum Id_ {
	Undef,
	BeginRem, EndRem,
	BeginRem2, EndRem2,
	BeginVar, EndVar,
	BeginText, EndText,
	BeginYuanyang, EndYuanyang,
	BeginCode, EndCode,
	BeginText2, EndText2,
	BeginBlock, EndBlock,
	Jvhao, Douhao,
	Maohao,
	Dunhao,
	For, Break, Continue,
	Range, Break2, Continue2,
	Return, Quit,
	If, Dengyu, Xiaoyudengyu, Xiaoyu, Dayudengyu, Dayu, Not,
		And, Or,
		Then, Else,
	Switch,
	Guandaodu, Guandaojie,
	Mod, ModFree, Name,
	Set, Alias, Def, Equ,
	Has,
	Eval, Load,
	Expl,
	Print,
	Exec,
	Brkpoint, ParBrkpoint,
	U2, U3, U4, U5, U6, U7, U9, U10, U11,

}

#[derive(Default, PartialEq, Debug)]
pub struct Grp_ {
	pub print_:bool,
	pub if_:bool,
	pub if2_:bool,
	pub set_:bool,
}

#[derive(PartialEq, Debug)]
pub struct Item_ {
	pub s_:String,
	pub id_:Id_,
	pub g_:Grp_,
}

impl Item_ {
	pub fn new(id:Id_, s:&str) -> Self {
		Self::new3(id, s, Default::default())
	}
	pub fn new_p(id:Id_, s:&str) -> Self {
		Self::new3(id, s, Grp_ {print_:true, ..Default::default()})
	}
	pub fn new_sp(id:Id_, s:&str) -> Self {
		Self::new3(id, s, Grp_ {print_:true, set_:true, ..Default::default()})
	}
	pub fn new_if(id:Id_, s:&str) -> Self {
		Self::new3(id, s, Grp_ {if_:true, ..Default::default()})
	}
	pub fn new_if2(id:Id_, s:&str) -> Self {
		Self::new3(id, s, Grp_ {if2_:true, ..Default::default()})
	}
	pub fn new2(id:Id_) -> Self {
		Self::new(id, "")
	}
	pub fn new3(id_:Id_, s:&str, g_:Grp_) -> Self {
		Self {s_:s.to_string(), id_, g_}
	}
}

pub struct Item2_ {
	pub s_:&'static str,
	pub id_:Vec<Id_>,
}

#[derive(Clone)]
pub struct List_ {
	pub undef_:RI_,

	pub begin_rem_:RI_,
	pub   end_rem_:RI_,
	pub begin_rem2_:RI_,
	pub   end_rem2_:RI_,
	pub begin_var_:RI_,
	pub   end_var_:RI_,
	pub begin_text_:RI_,
	pub   end_text_:RI_,
	pub begin_yuanyang_:RI_,
	pub   end_yuanyang_:RI_,
	pub begin_code_:RI_,
	pub   end_code_:RI_,
	pub begin_text2_:RI_,
	pub   end_text2_:RI_,
	pub begin_block_:RI_,
	pub   end_block_:RI_,
	pub jvhao_:RI_,
	pub douhao_:RI_,
	pub maohao_:RI_,
	pub dunhao_:RI_,
	pub for_:RI_,
	pub break_:RI_,
	pub continue_:RI_,
	pub range_:RI_,
	pub break2_:RI_,
	pub continue2_:RI_,
	pub return_:RI_,
	pub quit_:RI_,
	pub if_:RI_,
	pub dengyu_:RI_,
	pub xiaoyudengyu_:RI_,
	pub xiaoyu_:RI_,
	pub dayudengyu_:RI_,
	pub dayu_:RI_,
	pub not_:RI_,
	pub and_:RI_,
	pub or_:RI_,
	pub then_:RI_,
	pub else_:RI_,
	pub switch_:RI_,
	pub guandaodu_:RI_,
	pub guandaojie_:RI_,
	pub mod_:RI_,
	pub mod_free_:RI_,
	pub name_:RI_,
	pub set_:RI_,
	pub alias_:RI_,
	pub def_:RI_,
	pub equ_:RI_,
	pub has_:RI_,
	pub eval_:RI_,
	pub load_:RI_,
	pub expl_:RI_,
	pub print_:RI_,
	pub exec_:RI_,

	pub brkpoint_:RI_,
	pub par_brkpoint_:RI_,

	pub a_:Vec<RI_>,
	pub a2_:Vec<Rc_<Item2_>>,
}

impl List_ {
	pub fn new() -> Self {
		let begin_rem      = Rc_::new(Item_::new    (Id_::BeginRem,      "（"));
		let   end_rem      = Rc_::new(Item_::new    (Id_::EndRem,        "）"));
		let begin_rem2     = Rc_::new(Item_::new    (Id_::BeginRem2,     "【"));
		let   end_rem2     = Rc_::new(Item_::new    (Id_::EndRem2,       "】"));
		let begin_var      = Rc_::new(Item_::new    (Id_::BeginVar,      "‘"));
		let   end_var      = Rc_::new(Item_::new    (Id_::EndVar,        "’"));
		let begin_text     = Rc_::new(Item_::new    (Id_::BeginText,     "“"));
		let   end_text     = Rc_::new(Item_::new    (Id_::EndText,       "”"));
		let begin_yuanyang = Rc_::new(Item_::new    (Id_::BeginYuanyang, "下原样"));
		let   end_yuanyang = Rc_::new(Item_::new    (Id_::EndYuanyang,   "上原样"));
		let begin_code     = Rc_::new(Item_::new    (Id_::BeginCode,     "下代码"));
		let   end_code     = Rc_::new(Item_::new    (Id_::EndCode,       "上代码"));
		let begin_text2    = Rc_::new(Item_::new    (Id_::BeginText2,    "下文本"));
		let   end_text2    = Rc_::new(Item_::new    (Id_::EndText2,      "上文本"));
		let begin_block    = Rc_::new(Item_::new    (Id_::BeginBlock,    "先"));
		let   end_block    = Rc_::new(Item_::new    (Id_::EndBlock,      "了"));
		let jvhao          = Rc_::new(Item_::new    (Id_::Jvhao,         "。"));
		let douhao         = Rc_::new(Item_::new    (Id_::Douhao,        "，"));
		let maohao         = Rc_::new(Item_::new    (Id_::Maohao,        "："));
		let dunhao         = Rc_::new(Item_::new    (Id_::Dunhao,        "、"));
		let for1           = Rc_::new(Item_::new    (Id_::For,           "循环"));
		let break1         = Rc_::new(Item_::new    (Id_::Break,         "跳出"));
		let continue1      = Rc_::new(Item_::new    (Id_::Continue,      "再来"));
		let range          = Rc_::new(Item_::new    (Id_::Range,         "圈子"));
		let break2         = Rc_::new(Item_::new    (Id_::Break2,        "遁出"));
		let continue2      = Rc_::new(Item_::new    (Id_::Continue2,     "重来"));
		let return1        = Rc_::new(Item_::new    (Id_::Return,        "返回"));
		let quit           = Rc_::new(Item_::new    (Id_::Quit,          "结束"));
		let if1            = Rc_::new(Item_::new    (Id_::If,            "如果"));
		let dengyu         = Rc_::new(Item_::new    (Id_::Dengyu,        "等于"));
		let xiaoyudengyu   = Rc_::new(Item_::new    (Id_::Xiaoyudengyu,  "小于等于"));
		let xiaoyu         = Rc_::new(Item_::new    (Id_::Xiaoyu,        "小于"));
		let dayudengyu     = Rc_::new(Item_::new    (Id_::Dayudengyu,    "大于等于"));
		let dayu           = Rc_::new(Item_::new    (Id_::Dayu,          "大于"));
		let not            = Rc_::new(Item_::new    (Id_::Not,           "不"));
		let and            = Rc_::new(Item_::new    (Id_::And,           "并且"));
		let or             = Rc_::new(Item_::new    (Id_::Or,            "或者"));
		let then           = Rc_::new(Item_::new    (Id_::Then,          "那么"));
		let else1          = Rc_::new(Item_::new    (Id_::Else,          "否则"));
		let switch1        = Rc_::new(Item_::new    (Id_::Switch,        "分叉"));
		let guandaodu      = Rc_::new(Item_::new    (Id_::Guandaodu,     "管道堵"));
		let guandaojie     = Rc_::new(Item_::new    (Id_::Guandaojie,    "管道接"));
		let mod1           = Rc_::new(Item_::new    (Id_::Mod,           "模块"));
		let mod_free       = Rc_::new(Item_::new    (Id_::ModFree,       "释放模块"));
		let name           = Rc_::new(Item_::new    (Id_::Name,          "命名"));
		let set            = Rc_::new(Item_::new    (Id_::Set,           "赋予"));
		let alias          = Rc_::new(Item_::new    (Id_::Alias,         "别名"));
		let def            = Rc_::new(Item_::new    (Id_::Def,           "定义"));
		let equ            = Rc_::new(Item_::new    (Id_::Equ,           "以"));
		let has            = Rc_::new(Item_::new    (Id_::Has,           "存在"));
		let eval           = Rc_::new(Item_::new    (Id_::Eval,          "解释"));
		let load           = Rc_::new(Item_::new    (Id_::Load,          "加载"));
		let expl           = Rc_::new(Item_::new    (Id_::Expl,          "算术"));
		let print          = Rc_::new(Item_::new    (Id_::Print,         "显示"));
		let exec           = Rc_::new(Item_::new    (Id_::Exec,          "执行"));
		let brkpoint       = Rc_::new(Item_::new    (Id_::Brkpoint,      "这断点"));
		let par_brkpoint   = Rc_::new(Item_::new    (Id_::ParBrkpoint,   "这析断点"));

		Self {a2_:vec![],
			a_:vec![
				begin_rem.clone(),
				  end_rem.clone(),
				begin_rem2.clone(),
				  end_rem2.clone(),
				begin_var.clone(),
				  end_var.clone(),
				begin_text.clone(),
				  end_text.clone(),
				begin_yuanyang.clone(),
				  end_yuanyang.clone(),
				begin_code.clone(),
				  end_code.clone(),
				begin_text2.clone(),
				  end_text2.clone(),
				begin_block.clone(),
				  end_block.clone(),
				jvhao.clone(),
				douhao.clone(),
				maohao.clone(),
				dunhao.clone(),
				for1.clone(),
				break1.clone(),
				continue1.clone(),
				range.clone(),
				break2.clone(),
				continue2.clone(),
				return1.clone(),
				quit.clone(),
				if1.clone(),
				dengyu.clone(),
				xiaoyudengyu.clone(),
				xiaoyu.clone(),
				dayudengyu.clone(),
				dayu.clone(),
				not.clone(),
				and.clone(),
				or.clone(),
				then.clone(),
				else1.clone(),
				switch1.clone(),
				guandaodu.clone(),
				guandaojie.clone(),
				mod1.clone(),
				mod_free.clone(),
				name.clone(),
				set.clone(),
				alias.clone(),
				def.clone(),
				equ.clone(),
				has.clone(),
				eval.clone(),
				load.clone(),
				expl.clone(),
				print.clone(),
				exec.clone(),
				brkpoint.clone(),
				par_brkpoint.clone(),

			],
			undef_:Rc_::new(Item_::new2(Id_::Undef)),
			begin_rem_:begin_rem,
			  end_rem_:  end_rem,
			begin_rem2_:begin_rem2,
			  end_rem2_:  end_rem2,
			begin_var_:begin_var,
			  end_var_:  end_var,
			begin_text_:begin_text,
			  end_text_:  end_text,
			begin_yuanyang_:begin_yuanyang,
			  end_yuanyang_:  end_yuanyang,
			begin_code_:begin_code,
			  end_code_:  end_code,
			begin_text2_:begin_text2,
			  end_text2_:  end_text2,
			begin_block_:begin_block,
			  end_block_:  end_block,
			jvhao_:jvhao,
			douhao_:douhao,
			maohao_:maohao,
			dunhao_:dunhao,
			for_:for1,
			break_:break1,
			continue_:continue1,
			range_:range,
			break2_:break2,
			continue2_:continue2,
			return_:return1,
			quit_:quit,
			if_:if1,
			dengyu_:dengyu,
			xiaoyudengyu_:xiaoyudengyu,
			xiaoyu_:xiaoyu,
			dayudengyu_:dayudengyu,
			dayu_:dayu,
			not_:not,
			and_:and,
			or_:or,
			then_:then,
			else_:else1,
			switch_:switch1,
			guandaodu_:guandaodu,
			guandaojie_:guandaojie,
			mod_:mod1,
			mod_free_:mod_free,
			name_:name,
			set_:set,
			alias_:alias,
			def_:def,
			equ_:equ,
			has_:has,
			eval_:eval,
			load_:load,
			expl_:expl,
			print_:print,
			exec_:exec,
			brkpoint_:brkpoint,
			par_brkpoint_:par_brkpoint,

		}
	}

	pub fn add__(&mut self, s:&str, id:Id_) {
		self.a_.push(Rc_::new(Item_::new(id, s)))
	}
	pub fn add3__(&mut self, s:&str, id:Id_, g:Grp_) {
		self.a_.push(Rc_::new(Item_::new3(id, s, g)))
	}
	pub fn add2__(&mut self, s_:&'static str, id_:Vec<Id_>) {
		self.a2_.push(Rc_::new(Item2_ {id_, s_}))
	}
}
