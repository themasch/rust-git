pub struct Tree {
    entries: Vec<(String, Object)>
}

impl Tree {
    pub fn new() {
        Tree { entries: Vec::new() }
    }

    pub fn add(&mut self, name: String, obj: Object) -> Tree {
        entries.push((name, obj));
    }
}
