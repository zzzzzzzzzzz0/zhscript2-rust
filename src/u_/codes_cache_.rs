use super::Rc_;
use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::Hasher};

type Item_<T> = Rc_<T>;
pub type ORI_<'a, T> = Option<&'a Item_<T>>;

pub fn item__<T>(codes:T) -> Item_<T> {
	Rc_::new(codes)
}

#[derive(Default, Clone)]
pub struct List_<T> {
	a_:HashMap<u64, Item_<T>>,
}

impl<T> List_<T> {
	fn id__(&self, src:&str) -> u64 {
		let mut h = DefaultHasher::new();
		h.write(src.as_bytes());
		h.finish()
	}
	
	pub fn get__(&self, src:&str) -> ORI_<T> {
		self.a_.get(&self.id__(src))
	}
	pub fn set__(&mut self, src:&str, codes:T) {
		self.a_.insert(self.id__(src), item__(codes));
	}
}