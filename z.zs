#!/usr/lib/zhscript2-rust/l --。
赋予调试【顶】以。
定义提示、内容以下代码
	显示“# ‘内容’”换行。
上代码。
定义运行、命令、饶以下代码
	提示‘命令’。
	如果‘调试’那么先提示未‘参数0’。返回了。
	赋予错以执行‘命令’。
	提示‘错’。
	如果‘错’并且不‘饶’那么结束10。
上代码。
定义行解以下代码
	调用‘命令行解析’、‘解’、‘参数栈’。
上代码。

加载lib/gjke4。
加载lib/gjk4。
加载lib2/util.zs。

定义跳表、几以下代码
	循环【‘几’】制表符
上代码。

定义得id、1以下代码
	赋予id以调用‘大写方法’、‘1’、0、1。
	循环先
		赋予_以调用‘串位置’、‘id’、_。
		如果‘_’小于0那么跳出。
		赋予id以先调用‘串截取’、‘id’、0、‘_’了
			先调用‘大写方法’、先调用‘串截取’、‘id’、算术‘_’+1了、0、1了。
	了。
	如果是避讳‘id’那么
		赋予id以调用‘串截取’、‘id’、0、-1。
	‘id’。
上代码。
定义是避讳、id以下代码
	如果‘id’尾匹配1那么先
		赋予1以调用‘串截取’、‘id’、1、2。
		如果‘1’大于等于0并且小于等于9那么否则1。
	了。
上代码。

加载lib/clpars4。
赋予解以调用‘命令行加回调’、、
	kw、、0、下代码
		赋予量、最宽1、最宽2、最宽id以0。
		调用‘迭代三’、下代码
			赋予量【上】以算术‘量’+1。
			别名4、1、2、3以参数1、参数2、参数3、参数4。
			别名1/0以1。
			如果‘1/0’头匹配end那么
				赋予1/0以“  ”‘1/0’。
			如果‘4’那么赋予4以_‘4’。
			赋予数以算术4-调用‘串显长’、‘4’、-1。
			赋予4以‘4’循环【‘数’】“ ”。
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
			如果是避讳‘1/2’那么
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
		、for1、“循环”、-、
		、break1、“跳出”、-、
		、continue1、“再来”、、
		、range、“圈子”、-、
		、break2、“遁出”、-、
		、continue2、“重来”、、
		、return1、“返回”、-、
		、quit、“结束”、、
		、if1、“如果”、-、
		、dengyu、“等于”、-、
		、xiaoyudengyu、“小于等于”、-、
		、xiaoyu、“小于”、-、
		、dayudengyu、“大于等于”、-、
		、dayu、“大于”、-、
		、not、“不”、=、
		、and、“并且”、-、
		、or、“或者”、=、
		、then、“那么”、-、
		、else1、“否则”、、
		、switch1、“分叉”、、
		、switch2、“分支”、、
		、set、“赋予”、-、
		、alias、“别名”、-、
		、def、“定义”、-、
		、equ、“以”、、
		、has、“存在”、、
		、eval、“解释”、-、
		、load、“加载”、、
		、expl、“算术”、、
		、print、“显示”、、
		、exec、“执行”、、
		、mod1、“模块”、-、
		、mod_free、“释放模块”、-、
		、name、“命名”、、
		、guandaodu、“管道堵”、-、
		、guandaojie、“管道接”、-、
		、guandaojie2、“管道节”、、
		、brkpoint、“这断点”、前回-、
		、par_brkpoint、“这析断点”、、
		、u2、、-、
		、u3、、-、
		、u4、、-、
		、u5、、-、
		、u6、、-、
		、u7、、-、
		、u9、、-、
		、u10、、-、
		、u11、、。

		显示“// ‘量’ ‘最宽1’,‘最宽2’,‘最宽id’”换行。
		循环【‘量’】【次】先
			赋予数以算术‘最宽1’-‘‘次’/1/1’。
			赋予“ ‘次’/1”以循环【‘数’】“ ”。
			赋予数以算术‘最宽id’-‘‘次’id1’。
			赋予“ ‘次’id1”以循环【‘数’】“ ”。
		了。

		显示下原样
#![allow(dead_code)]

use super::Rc_;

pub type RI_ = Rc_<Item_>;
pub type ORI_ = Option<RI_>;

#[derive(PartialEq, Debug)]
pub enum Id_ {
上原样。
		循环【‘量’】【次】先
			别名3以‘次’/3。
			如果‘3’内包含别那么再来。
			赋予下以如果‘3’内包含=那么2否则1。
			显示分支‘次’先
				1先跳表‘下’了。
			了‘‘次’id’,分支‘次’先
				‘量’：换行。
				如果‘3’内包含-那么“ ”否则换行先跳表‘下’了
			了。
		了。
		显示下原样
}

#[derive(Default, PartialEq, Debug)]
pub struct Grp_ {
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
上原样。
		循环【‘量’】【次】先
			如果‘‘次’/2’等于“”那么再来。
			别名3以‘次’/3。
			显示先
				如果‘3’内包含前回那么换行。
				跳表1
			了“pub ”‘‘次’/1_’:RI_,换行先
				如果‘3’内包含后回那么换行。
			了。
		了。
		显示下原样
	pub a_:Vec<RI_>,
	pub a2_:Vec<Rc_<Item2_>>,
}

impl List_ {
	pub fn new() -> Self {
上原样。
		循环【‘量’】【次】先
			别名2以‘次’/2。
			如果不‘2’那么再来。
			别名4以‘次’/4。
			显示先跳表2了“let ‘‘次’/1’‘ ‘次’/1’ = Rc_::new(Item_::new‘4’(Id_::‘‘次’id’, ‘ ‘次’id1’"”‘2’"));换行。
		了。
		显示下原样
		Self {a2_:vec![],
			a_:vec![
上原样。
		循环【‘量’】【次】先
			如果不‘‘次’/2’那么再来。
			显示先跳表4了‘‘次’/1’.clone(),换行。
		了。
		显示下原样
			],
上原样。
		循环【‘量’】【次】先
			别名1_以‘次’/1_。
			分支‘‘次’/2’先
				先了再来。
				0先
					显示先跳表3了‘1_’:Rc_::new(Item_::new2(Id_::‘‘次’id’)),换行。
					再来。
				了。
			了。
			显示先跳表3了‘1_’:‘‘次’/1’,换行。
		了。
		显示下原样
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
上原样。
	上代码、
	kw2、、0、下代码
		加载lib/doscmd4。
		赋予tmp以/tmp/keyword_.rs。
		调用‘echo’、‘tmp’。
		定义“显示”以下代码
			调用‘echo’、-ma、‘参数1’、‘tmp’。
		上代码。
		行解kw。
		执行“meld ‘tmp’ src/u_/keyword_.rs”。
	上代码、
	br|b、下代码
		运行“rust.zs2 ‘参数0’”
	上代码、0、、
	(r)(\d+)、、r、下代码
		运行“cargo ‘参数1’ /zzzzzzzzzzz4/home/zzzzzzzzzzz/test/rust-zs2/”分叉‘参数2’先
			1“_1_”。
			2“text-2”。
			先
				显示啊循环【6】先“~ ”了够不够悠长换行。
				结束1
			了
		了.zs
	上代码、
	-h2、“赋予调试【顶】以1。”、0、、
	#、、h、。
行解‘参数栈’。
