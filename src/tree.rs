use object::Object;

pub struct Tree<'a> {
    entries: Vec<(String, &'a Object)>
}

impl<'a> Tree<'a> {
    pub fn new() -> Tree<'a> {
        Tree { entries: Vec::new() }
    }

    pub fn add(&mut self, name: String, obj: &'a Object) -> &Tree {
        self.entries.push((name, obj));
        self
    }
}

impl<'a> Object for Tree<'a> {
    fn get_hash_content(&self) -> Vec<u8> {
        let buffer = Vec::new();
        buffer.extend_from_slice(format!("blob {}", self.content_length()).as_bytes());
    }
}
