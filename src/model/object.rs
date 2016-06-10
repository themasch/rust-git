use model::Blob;
use model::Tree;
use model::hashable::Hashable;

#[derive(Debug)]
pub enum Object {
    Blob(Blob),
    Tree(Tree),
}

impl From<Blob> for Object {
    fn from(blob: Blob) -> Object {
        Object::Blob(blob)
    }
}

impl From<Tree> for Object {
    fn from(tree: Tree) -> Object {
        Object::Tree(tree)
    }
}

impl Hashable for Object {
    fn get_hash_content(&self) -> Vec<u8> {
        match self {
            &Object::Blob(ref b) => b.get_hash_content(),
            &Object::Tree(ref t) => t.get_hash_content()
        }
    }
}
