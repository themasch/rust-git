use model::hashable::Hashable;
use model::tree_node::TreeNode;

#[derive(Debug,Clone)]
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

impl<'a> Hashable for Tree<'a> {
    // this is terrible slow. and not just slow, its just pure terrible
    fn get_hash_content(&self) -> Vec<u8> {
        let buffer = self.entries.iter()
            .fold(Vec::new(), | mut buffer, entry | {
                //format: [mode in ascii octal]0x20[name as utf8 string]\0x00[sha1 hash]
                buffer.extend_from_slice(format!("{:o}", entry.mode).as_bytes());
                buffer.push(0x20);
                buffer.extend_from_slice(entry.name.as_bytes());
                buffer.push(0x00);
                buffer.extend(entry.object.hash());

                return buffer;
            });
        let mut result = format!("blob {}", buffer.len()).into_bytes();
        result.push(0);
        result.extend(buffer);

        return result;
    }
}
