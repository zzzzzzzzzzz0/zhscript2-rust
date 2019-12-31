// 66 14,8,13

use std::rc::Rc;

pub type RI_ = Rc<Item_>;
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
	Mod, Name,
	Set, Alias, Def, Equ,
	Has,
	Eval, Load,
	Expl,
	Print,
	Exec,
	Brkpoint, ParBrkpoint,
	U2, U3, U4, U5, U6, U7, U9, U10,

}

#[derive(Debug)]
pub struct Grp_ {
	pub print_:bool,
	pub if_:bool,
	pub if2_:bool,
	pub set_:bool,
}

#[derive(Debug)]
pub struct Item_ {
	pub s_:String,
	pub id_:Id_,
	pub g_:Grp_,
}

impl Item_ {
	pub fn new(id:Id_, s:&str) -> Self {
		Self::new3(id, s, Grp_ {print_:false, if_:false, if2_:false, set_:false})
	}
	pub fn new_p(id:Id_, s:&str) -> Self {
		Self::new3(id, s, Grp_ {print_:true, if_:false, if2_:false, set_:false})
	}
	pub fn new_sp(id:Id_, s:&str) -> Self {
		Self::new3(id, s, Grp_ {print_:true, if_:false, if2_:false, set_:true})
	}
	pub fn new_if(id:Id_, s:&str) -> Self {
		Self::new3(id, s, Grp_ {print_:false, if_:true, if2_:false, set_:false})
	}
	pub fn new_if2(id:Id_, s:&str) -> Self {
		Self::new3(id, s, Grp_ {print_:false, if2_:true, if_:false, set_:false})
	}
	pub fn new2(id:Id_) -> Self {
		Self::new(id, "")
	}
	pub fn new3(id:Id_, s:&str, g:Grp_) -> Self {
		Self {s_:s.to_string(), id_:id, g_:g}
	}
}

pub struct List_ {
	pub undef_:Rc<Item_>,

	pub begin_rem_:Rc<Item_>,
	pub   end_rem_:Rc<Item_>,
	pub begin_rem2_:Rc<Item_>,
	pub   end_rem2_:Rc<Item_>,
	pub begin_var_:Rc<Item_>,
	pub   end_var_:Rc<Item_>,
	pub begin_text_:Rc<Item_>,
	pub   end_text_:Rc<Item_>,
	pub begin_yuanyang_:Rc<Item_>,
	pub   end_yuanyang_:Rc<Item_>,
	pub begin_code_:Rc<Item_>,
	pub   end_code_:Rc<Item_>,
	pub begin_text2_:Rc<Item_>,
	pub   end_text2_:Rc<Item_>,
	pub begin_block_:Rc<Item_>,
	pub   end_block_:Rc<Item_>,
	pub jvhao_:Rc<Item_>,
	pub douhao_:Rc<Item_>,
	pub maohao_:Rc<Item_>,
	pub dunhao_:Rc<Item_>,
	pub dunhao2_:Rc<Item_>,
	pub for_:Rc<Item_>,
	pub break_:Rc<Item_>,
	pub continue_:Rc<Item_>,
	pub range_:Rc<Item_>,
	pub break2_:Rc<Item_>,
	pub continue2_:Rc<Item_>,
	pub return_:Rc<Item_>,
	pub quit_:Rc<Item_>,
	pub if_:Rc<Item_>,
	pub dengyu_:Rc<Item_>,
	pub xiaoyudengyu_:Rc<Item_>,
	pub xiaoyu_:Rc<Item_>,
	pub dayudengyu_:Rc<Item_>,
	pub dayu_:Rc<Item_>,
	pub not_:Rc<Item_>,
	pub and_:Rc<Item_>,
	pub or_:Rc<Item_>,
	pub then_:Rc<Item_>,
	pub else_:Rc<Item_>,
	pub switch_:Rc<Item_>,
	pub guandaodu_:Rc<Item_>,
	pub guandaojie_:Rc<Item_>,
	pub mod_:Rc<Item_>,
	pub name_:Rc<Item_>,
	pub set_:Rc<Item_>,
	pub alias_:Rc<Item_>,
	pub def_:Rc<Item_>,
	pub equ_:Rc<Item_>,
	pub has_:Rc<Item_>,
	pub eval_:Rc<Item_>,
	pub load_:Rc<Item_>,
	pub expl_:Rc<Item_>,
	pub print_:Rc<Item_>,
	pub exec_:Rc<Item_>,

	pub brkpoint_:Rc<Item_>,
	pub par_brkpoint_:Rc<Item_>,

	pub a_:Vec<Rc<Item_>>,
}

impl List_ {
	pub fn new() -> Self {
		let begin_rem      = Rc::new(Item_::new    (Id_::BeginRem,      "（"));
		let   end_rem      = Rc::new(Item_::new    (Id_::EndRem,        "）"));
		let begin_rem2     = Rc::new(Item_::new    (Id_::BeginRem2,     "【"));
		let   end_rem2     = Rc::new(Item_::new    (Id_::EndRem2,       "】"));
		let begin_var      = Rc::new(Item_::new    (Id_::BeginVar,      "‘"));
		let   end_var      = Rc::new(Item_::new    (Id_::EndVar,        "’"));
		let begin_text     = Rc::new(Item_::new    (Id_::BeginText,     "“"));
		let   end_text     = Rc::new(Item_::new    (Id_::EndText,       "”"));
		let begin_yuanyang = Rc::new(Item_::new    (Id_::BeginYuanyang, "下原样"));
		let   end_yuanyang = Rc::new(Item_::new    (Id_::EndYuanyang,   "上原样"));
		let begin_code     = Rc::new(Item_::new    (Id_::BeginCode,     "下代码"));
		let   end_code     = Rc::new(Item_::new    (Id_::EndCode,       "上代码"));
		let begin_text2    = Rc::new(Item_::new    (Id_::BeginText2,    "下文本"));
		let   end_text2    = Rc::new(Item_::new    (Id_::EndText2,      "上文本"));
		let begin_block    = Rc::new(Item_::new    (Id_::BeginBlock,    "先"));
		let   end_block    = Rc::new(Item_::new    (Id_::EndBlock,      "了"));
		let jvhao          = Rc::new(Item_::new    (Id_::Jvhao,         "。"));
		let douhao         = Rc::new(Item_::new    (Id_::Douhao,        "，"));
		let maohao         = Rc::new(Item_::new    (Id_::Maohao,        "："));
		let dunhao         = Rc::new(Item_::new    (Id_::Dunhao,        "、"));
		let dunhao2        = Rc::new(Item_::new    (Id_::Dunhao,        "为"));
		let for1           = Rc::new(Item_::new    (Id_::For,           "循环"));
		let break1         = Rc::new(Item_::new    (Id_::Break,         "跳出"));
		let continue1      = Rc::new(Item_::new    (Id_::Continue,      "再来"));
		let range          = Rc::new(Item_::new    (Id_::Range,         "圈子"));
		let break2         = Rc::new(Item_::new    (Id_::Break2,        "遁出"));
		let continue2      = Rc::new(Item_::new    (Id_::Continue2,     "重来"));
		let return1        = Rc::new(Item_::new_p  (Id_::Return,        "返回"));
		let quit           = Rc::new(Item_::new_p  (Id_::Quit,          "结束"));
		let if1            = Rc::new(Item_::new    (Id_::If,            "如果"));
		let dengyu         = Rc::new(Item_::new_if (Id_::Dengyu,        "等于"));
		let xiaoyudengyu   = Rc::new(Item_::new_if (Id_::Xiaoyudengyu,  "小于等于"));
		let xiaoyu         = Rc::new(Item_::new_if (Id_::Xiaoyu,        "小于"));
		let dayudengyu     = Rc::new(Item_::new_if (Id_::Dayudengyu,    "大于等于"));
		let dayu           = Rc::new(Item_::new_if (Id_::Dayu,          "大于"));
		let not            = Rc::new(Item_::new_if (Id_::Not,           "不"));
		let and            = Rc::new(Item_::new_if (Id_::And,           "并且"));
		let or             = Rc::new(Item_::new_if (Id_::Or,            "或者"));
		let then           = Rc::new(Item_::new_if2(Id_::Then,          "那么"));
		let else1          = Rc::new(Item_::new_if2(Id_::Else,          "否则"));
		let switch1        = Rc::new(Item_::new    (Id_::Switch,        "分叉"));
		let guandaodu      = Rc::new(Item_::new    (Id_::Guandaodu,     "管道堵"));
		let guandaojie     = Rc::new(Item_::new    (Id_::Guandaojie,    "管道接"));
		let mod1           = Rc::new(Item_::new_p  (Id_::Mod,           "模块"));
		let name           = Rc::new(Item_::new_p  (Id_::Name,          "命名"));
		let set            = Rc::new(Item_::new_sp (Id_::Set,           "赋予"));
		let alias          = Rc::new(Item_::new_sp (Id_::Alias,         "别名"));
		let def            = Rc::new(Item_::new_sp (Id_::Def,           "定义"));
		let equ            = Rc::new(Item_::new    (Id_::Equ,           "以"));
		let has            = Rc::new(Item_::new    (Id_::Has,           "存在"));
		let eval           = Rc::new(Item_::new_p  (Id_::Eval,          "解释"));
		let load           = Rc::new(Item_::new_p  (Id_::Load,          "加载"));
		let expl           = Rc::new(Item_::new    (Id_::Expl,          "算术"));
		let print          = Rc::new(Item_::new_p  (Id_::Print,         "显示"));
		let exec           = Rc::new(Item_::new_p  (Id_::Exec,          "执行"));
		let brkpoint       = Rc::new(Item_::new    (Id_::Brkpoint,      "这断点"));
		let par_brkpoint   = Rc::new(Item_::new    (Id_::ParBrkpoint,   "这析断点"));

		Self {a_:vec![
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
				dunhao2.clone(),
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
			undef_:Rc::new(Item_::new2(Id_::Undef)),
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
			dunhao2_:dunhao2,
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
}
