// Create custom error type
use std::fmt::{Debug, Display, Formatter};

pub struct RemoveError {}

impl Debug for RemoveError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}
impl Display for RemoveError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl std::error::Error for RemoveError {}
impl RemoveError {
	fn new() -> Self {
		Self {}
	}
}

// Main dictionary code
pub struct Dictionary {
	pub keys: Vec<String>,
	pub values: Vec<String>
}

impl Default for Dictionary {
	fn default() -> Dictionary {
		Dictionary::new()
	}
}

impl Dictionary {
	pub fn new() -> Dictionary {
		Dictionary {
			keys: Vec::new(),
			values: Vec::new()
		}
	}
	pub fn set(&mut self, k: String, v: String) {
		self.keys.push(k);
		self.values.push(v);
	}
	pub fn get(&self, k: &str) -> Option<&String> {
		let keys: &Vec<String> = &self.keys;
		let i = &keys.iter().position(|v| v == k);
		if let Some(n) = i {
			Some(&self.values[*n])
		} else {
			None
		}
	}
	pub fn get_key(&self, val: &str) -> Option<&String> {
		let values: &Vec<String> = &self.values;
		let i = &values.iter().position(|v| v == val);
		if let Some(n) = i {
			Some(&self.keys[*n])
		} else {
			None
		}
	}
	pub fn remove(&mut self, k: &str) -> Result<(), RemoveError> {
		let keys: &mut Vec<String> = &mut self.keys;
		let values: &mut Vec<String> = &mut self.values;
		let i = keys.iter().position(|v| v == k);
		if let Some(n) = i {
			keys.remove(n);
			values.remove(n);
			Ok(())
		} else {
			Err(RemoveError::new())
		}
	}
}


// Iterator implementation
pub struct DictIter<'a> {
	dict: &'a Dictionary,
	pos: usize
}
impl Dictionary {
	pub fn iter(&self) -> DictIter {
		DictIter { dict: self, pos: 0 }
	}
}

impl<'a> Iterator for DictIter<'a> {
	type Item = (&'a String, &'a String);

	fn next(&mut self) -> Option<Self::Item> {
		let pos = self.pos;
		let keys = &self.dict.keys;
		let values = &self.dict.values;

		if pos >= keys.len() {
			return None;
		}

		let next_key = &keys[pos];
		let next_value = &values[pos];
		self.pos += 1;
		Some((next_key, next_value))
	}
}

#[cfg(test)]
mod tests {
	use crate::Dictionary;
	#[test]
	fn iterator_test() {
		let mut dict: Dictionary = Dictionary::new();
		dict.set("key".to_string(), "value".to_string());
		dict.set("key2".to_string(), "value2".to_string());
		dict.set("key3".to_string(), "value3".to_string());

		for (k, v) in dict.iter() {
			println!("{}, {}", k, v)
		}
	}
}
