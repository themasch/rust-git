use model::hashable::Hashable;

#[derive(Debug,Clone)]
pub struct TreeNode<'b> {
    pub mode: u16,
    pub object: &'b Hashable,
    pub name: String
}

impl<'b> TreeNode<'b> {
    pub fn new(node: &'b Hashable, mode: u16, name: String) -> TreeNode<'b> {
        TreeNode { object: node, mode: mode, name: name }
    }
}
