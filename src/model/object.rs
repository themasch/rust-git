use model::Blob;
use model::Tree;
#[derive(Debug,Clone)]
pub enum Object<'a> {
    Blob(Blob),
    Tree(Tree<'a>),
}
