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
    pub fn remove(&mut self, k: &str) -> Option<()> {
        let keys: &mut Vec<String> = &mut self.keys;
        let values: &mut Vec<String> = &mut self.values;
        let i = keys.iter().position(|v| v == k)?;
        keys.remove(i);
        values.remove(i);
        Some(())
    }
}

#[cfg(test)]
mod tests {}
