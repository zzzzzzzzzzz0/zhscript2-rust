pub const ERR_:i32 = 1000;
/*
11xxx jump
3xxx if
*/

pub fn err__(s:String) -> Item_ {
	err3__(ERR_, s)
}
pub fn err2__(s:&str) -> Item_ {err__(s.to_string())}
pub fn err3__(i:i32, s:String) -> Item_ {err5__(i, 0, s)}
pub fn err4__(i:i32, i2:i32, s:String, s2:String) -> Item_ {
	Err((i, i2, s, s2))
}
pub fn err5__(i:i32, i2:i32, s:String) -> Item_ {err4__(i, i2, s, "".to_string())}
pub fn qve__() -> Item_ {err2__("缺")}

pub fn n__(i:i32) -> Item_ {n2__(i, 0)}
pub fn n2__(i:i32, i2:i32) -> Item_ {err4__(i, i2, "".to_string(), "".to_string())}

pub fn exitcode__(i:i32) -> i32 {
	if i >= 0 && i <= 255 {i} else {254}
}

pub fn eprtn__(_:i32, _:i32, s:&String, s2:&String) {
	eprintln!("{}{}", s, s2);
}

pub type Err_ = (i32, i32, String, String);
pub type Item_ = Result<(), Err_>;

pub fn item__(i:Item_, mut f:impl FnMut(Item_) -> Item_) -> Item_ {
	f(i)
}
