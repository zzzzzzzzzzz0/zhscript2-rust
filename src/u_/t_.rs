use std::rc::Rc;
use std::io::Write;

pub fn or__<'a, T>(o:&'a Option<Rc<T>>) -> &'a T {
	o.as_ref().unwrap()
}
pub fn o__<T>(o:&Option<T>) -> &T {
	o.as_ref().unwrap()
}
pub fn some__<T>(o:&Option<T>) -> Option<&T> {
	if let Some(o) = o.as_ref() {
		Some(o)
	} else {
		None
	}
}

pub fn s2n__<T:std::str::FromStr>(s:&str) -> Option<T> {
	if let Ok(i) = s.parse::<T>() {Some(i)} else {None}
}

pub fn i2u__(i:i32, max:usize) -> usize {
	if i >= 0 {i as usize} else {
		let i = -i as usize;
		if max >= i {
			max - i
		} else {
			0
		}
	}
}

pub fn pf__() {
	std::io::stdout().flush().unwrap()
}
	
pub fn b__<'a>(b:bool) -> &'a str {
	if b {"y"} else {"n"}
}

pub fn with__(cs:&[char], s2:&str, from:usize) -> Option<usize> {
	let cs2:Vec<char> = s2.chars().collect();
	let mut idx2 = 0;
	loop {
		let idx = from + idx2;
		if idx2 >= cs2.len() {
			return Some(idx2)
		}
		if idx >= cs.len() {
			break
		}
		if cs[idx] != cs2[idx2] {
			break
		}
		idx2 += 1;
	}
	None
}