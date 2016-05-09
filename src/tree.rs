use object::Object;
use tree_node::TreeNode;

pub struct Tree<'a> {
    entries: Vec<&'a TreeNode<'a>>
}

impl<'a> Tree<'a> {
    pub fn new() -> Tree<'a> {
        Tree { entries: Vec::new() }
    }

    pub fn add(&mut self, obj: &'a TreeNode) -> &Tree {
        self.entries.push(obj);
        self
    }
}

impl<'a> Object for Tree<'a> {
    // this is terrible slow. and not just slow, its just pure terrible
    fn get_hash_content(&self) -> Vec<u8> {
        let buffer = self.entries.iter()
            .fold(Vec::new(), | mut buffer, entry | {
                //buffer.extend(entry.get_mode_octal());
                buffer.extend_from_slice(format!("{:o}", entry.mode).as_bytes());
                buffer.extend_from_slice(" ".as_bytes());
                buffer.extend_from_slice(entry.name.as_bytes());
                buffer.push(0);
                buffer.extend(entry.object.hash());

                return buffer;
            });
        let mut result = format!("blob {}", buffer.len()).into_bytes();
        result.push(0);
        result.extend(buffer);

        return result;
    }
}
