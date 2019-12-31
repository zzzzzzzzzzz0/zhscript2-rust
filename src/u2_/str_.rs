#![allow(dead_code)]

pub fn split__(s:&str) -> Vec<String> {
	split2__(s, " \t\r\n", false, false, false)
}

pub fn split2__(s:&str, s3:&str, sp_yes:bool, yange:bool, yangend:bool) -> Vec<String> {
	let mut v = vec![];
	let mut s2 = String::new();
	let mut a = s.chars();
	let mut b = false;
	while let Some(i) = a.next() {
		if s3.contains(i) {
			if yange || b {
				v.push(s2.to_string());
			}
			s2.clear();
			b = false;
			continue;
		}
		match i {
			'"' | '\'' => {
				while let Some(i2) = a.next() {
					if i2 == i {
						break;
					}
					s2 += &i2.to_string();
				}
				b = true;
				continue;
			}
			' ' | '\t' | '\r' | '\n' => if !sp_yes {
				continue;
			}
			_ => {}
		}
		s2 += &i.to_string();
		b = true;
	}
	if yange && yangend || b {
		v.push(s2.to_string());
	}
	v
}

#[cfg(test)]
mod test_ {
	use super::*;
	const S:&str = "1| |||";
	const S3:&str = "|";
	#[test]
	fn yange_false__() {
		let v = split2__(S, S3, false, false, false);
		assert_eq!(v.len(), 1)
	}
	#[test]
	fn yange__() {
		let v = split2__(S, S3, false, true, true);
		assert_eq!(v.len(), 5)
	}
	#[test]
	fn yangend__() {
		let v = split2__(S, S3, false, true, false);
		assert_eq!(v.len(), 4)
	}
}
