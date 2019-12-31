use std::collections::HashMap;
use std::rc::Rc;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

type Item_<T> = Rc<T>;
pub type ORI_<'a, T> = Option<&'a Item_<T>>;
pub type OI_<T> = Option<Item_<T>>;

pub fn item__<T>(codes:T) -> Item_<T> {
	Rc::new(codes)
}

pub struct List_<T> {
	a_:HashMap<u64, Item_<T>>,
}

impl<T> List_<T> {
	pub fn new() -> Self {Self {a_:HashMap::new()}}
	
	fn id__(&self, src:&str) -> u64 {
		let mut h = DefaultHasher::new();
		h.write(src.as_bytes());
		h.finish()
	}
	
	pub fn get__(&mut self, src:&str) -> ORI_<T> {
		self.a_.get(&self.id__(src))
	}
	pub fn set__(&mut self, src:&str, codes:T) {
		self.a_.insert(self.id__(src), item__(codes));
	}
}