#!/usr/lib/zhscript2/l --。
赋予调试【顶】以。
定义提示、内容以下代码
	显示“# ‘内容’”换行。
上代码。
定义运行、命令、饶以下代码
	提示‘命令’。
	如果‘调试’那么提示未‘参数0’，退出。
	赋予错以执行‘命令’。
	提示‘错’。
	如果‘错’并且不‘饶’那么结束10。
上代码。
定义行解以下代码
	调用‘命令行解析’、‘参数栈’。
上代码。

加载lib/gjke4。
加载lib/gjk4。
加载lib2/util.zs。

定义跳表、几、下【隔】以下代码
	循环‘几’制表符
上代码。

定义得id、1以下代码
	赋予id以调用‘大写方法’、‘1’、0、1。
	循环先
		赋予_以调用‘串位置’、‘id’、_。
		如果‘_’小于0那么跳出。
		赋予id以先调用‘串截取’、‘id’、0、‘_’了
			先调用‘大写方法’、先调用‘串截取’、‘id’、算术‘_’+1了、0、1了。
	了。
	如果‘id’尾匹配1那么
		赋予id以调用‘串截取’、‘id’、0、-1。
	‘id’。
上代码。

加载lib/clpars4。
调用‘命令行加回调’、
	kw、、0、下代码
		赋予量、最宽1、最宽2、最宽id以0。
		调用‘迭代三’、下代码
			赋予量【上】以算术‘量’+1。
			别名4、1、2、3以参数1、参数2、参数3、参数4。
			别名1/0以1。
			如果‘1/0’头匹配end那么
				赋予1/0以“  ”‘1/0’。
			如果‘4’那么赋予4以_‘4’。
			赋予4以‘4’循环先算术4-调用‘串显长’、‘4’、-1了“ ”。
			赋予‘量’/1【上】、‘量’/2【上】、‘量’/3【上】、‘量’/4【上】以‘1/0’、‘2’、‘3’、‘4’。

			赋予宽以调用‘串显长’、‘1/0’、-1。
			如果‘最宽1’小于‘宽’那么
				赋予最宽1【上】以‘宽’。
			赋予‘量’/1/1【上】以‘宽’。

			赋予宽以调用‘串显长’、‘2’、-1。
			如果‘最宽2’小于‘宽’那么
				赋予最宽2【上】以‘宽’。
			赋予‘量’/2/1【上】以‘宽’。

			赋予id以得id‘1’。
			如果‘3’内包含别那么
				赋予id以调用‘串截取’、‘id’、0、-1。
			赋予‘量’id【上】以‘id’。

			赋予宽以调用‘串显长’、‘id’、-1。
			如果‘最宽id’小于‘宽’那么
				赋予最宽id【上】以‘宽’。
			赋予‘量’id1【上】以‘宽’。
			
			别名1/2以1/0。
			如果‘1/2’尾匹配1那么
				赋予1/2以调用‘串截取’、‘1/2’、0、-1。
			赋予‘量’/1_【上】以‘1/2’_。
		上代码、4、
		、undef、0、后回、
		、begin_rem、“（”、-、
		、end_rem、“）”、、
		、begin_rem2、“【”、-、
		、end_rem2、“】”、、
		、begin_var、下原样‘上原样、-、
		、end_var、下原样’上原样、、
		、begin_text、下原样“上原样、-、
		、end_text、下原样”上原样、、
		、begin_yuanyang、“下原样”、-、
		、end_yuanyang、“上原样”、、
		、begin_code、“下代码”、-、
		、end_code、“上代码”、、
		、begin_text2、“下文本”、-、
		、end_text2、“上文本”、、
		、begin_block、“先”、-、
		、end_block、“了”、、
		、jvhao、“。”、-、
		、douhao、“，”、、
		、maohao、“：”、、
		、dunhao、“、”、、
		、dunhao2、“为”、别、
		、for1、“循环”、-、
		、break1、“跳出”、-、
		、continue1、“再来”、、
		、range、“圈子”、-、
		、break2、“遁出”、-、
		、continue2、“重来”、、
		p、return1、“返回”、-、
		p、quit、“结束”、、
		、if1、“如果”、-、
		if、dengyu、“等于”、-、
		if、xiaoyudengyu、“小于等于”、-、
		if、xiaoyu、“小于”、-、
		if、dayudengyu、“大于等于”、-、
		if、dayu、“大于”、-、
		if、not、“不”、=、
		if、and、“并且”、-、
		if、or、“或者”、=、
		if2、then、“那么”、-、
		if2、else1、“否则”、、
		、switch1、“分叉”、、
		、guandaodu、“管道堵”、-、
		、guandaojie、“管道接”、、
		p、mod1、“模块”、-、
		p、name、“命名”、、
		sp、set、“赋予”、-、
		sp、alias、“别名”、-、
		sp、def、“定义”、-、
		、equ、“以”、、
		、has、“存在”、、
		p、eval、“解释”、-、
		p、load、“加载”、、
		、expl、“算术”、、
		p、print、“显示”、、
		p、exec、“执行”、、
		、brkpoint、“这断点”、前回-、
		、par_brkpoint、“这析断点”、、
		、u2、、-、
		、u3、、-、
		、u4、、-、
		、u5、、-、
		、u6、、-、
		、u7、、-、
		、u9、、-、
		、u10、、。

		显示“// ‘量’ ‘最宽1’,‘最宽2’,‘最宽id’”换行。
		循环‘量’次先
			赋予“ ‘次’/1”以循环先算术‘最宽1’-‘‘次’/1/1’了“ ”。
			赋予“ ‘次’id1”以循环先算术‘最宽id’-‘‘次’id1’了“ ”。
		了。

		显示下原样
use std::rc::Rc;

pub type RI_ = Rc<Item_>;
pub type ORI_ = Option<RI_>;

#[derive(PartialEq, Debug)]
pub enum Id_ {
上原样。
		循环‘量’次先
			别名3以‘次’/3。
			如果‘3’内包含别那么再来。
			赋予下以如果‘3’内包含=那么2否则1。
			显示分支‘次’先
				1：跳表‘下’下。
			了‘‘次’id’,分支‘次’先
				‘量’：换行。
				如果‘3’内包含-那么“ ”否则换行先跳表‘下’下了
			了。
		了。
		显示下原样
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
上原样。
		循环‘量’次先
			如果‘‘次’/2’等于“”那么再来。
			别名3以‘次’/3。
			显示先
				如果‘3’内包含前回那么换行。
				跳表1下
			了“pub ”‘‘次’/1_’:Rc<Item_>,换行先
				如果‘3’内包含后回那么换行。
			了。
		了。
		显示下原样
	pub a_:Vec<Rc<Item_>>,
}

impl List_ {
	pub fn new() -> Self {
上原样。
		循环‘量’次先
			别名2以‘次’/2。
			如果不‘2’那么再来。
			别名4以‘次’/4。
			显示先跳表2下了“let ‘‘次’/1’‘ ‘次’/1’ = Rc::new(Item_::new‘4’(Id_::‘‘次’id’, ‘ ‘次’id1’"”‘2’"));换行。
		了。
		显示下原样
		Self {a_:vec![
上原样。
		循环‘量’次先
			如果不‘‘次’/2’那么再来。
			显示先跳表4下了‘‘次’/1’.clone(),换行。
		了。
		显示下原样
			],
上原样。
		循环‘量’次先
			别名1_以‘次’/1_。
			分支‘‘次’/2’先
				：再来。
				0先
					显示先跳表3下了‘1_’:Rc::new(Item_::new2(Id_::‘‘次’id’)),换行。
					再来。
				了。
			了。
			显示先跳表3下了‘1_’:‘‘次’/1’,换行。
		了。
		显示下原样
		}
	}
}
上原样。
	上代码、
	br、下代码
		运行“rust.zs2 ‘参数0’”
	上代码、0、、
	mvr、下代码
		运行“mv ./target/release/zhscript2 /zzzzzzzzzzz4/usr/lib/zhscript2-rust/l2”
	上代码、0、、
	-h2、“赋予调试【顶】以1。”、0、、
	#、、h、。
行解‘参数栈’。
