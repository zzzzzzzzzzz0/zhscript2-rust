pub const       ERR_:i32 = 1000;
/*
2xxx jump
3xxx if
*/

pub fn err__(s:String) -> Item_ {
	Err((ERR_, s, "".to_string()))
}
pub fn err2__(s:&str) -> Item_ {err__(s.to_string())}
pub fn qve__() -> Item_ {err2__("缺")}
#[allow(dead_code)]
pub fn keng__() -> Item_ {err2__("坑")}

pub fn n__(i:i32) -> Item_ {
	Err((i, "".to_string(), "".to_string()))
}

pub fn exitcode__(i:i32) -> i32 {
	if i >= 0 && i <= 255 {i} else {254}
}

pub type Item_ = Result<(), (i32, String, String)>;

pub fn item__(i:Item_, mut f:impl FnMut(Item_) -> Item_) -> Item_ {
	f(i)
}
