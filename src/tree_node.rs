use object::Object;

pub struct TreeNode<'a> {
    pub mode: u16,
    pub object: &'a Object,
    pub name: String
}

impl<'a>  TreeNode<'a> {
    pub fn new(node: &'a Object, mode: u16, name: String) -> TreeNode<'a> {
        TreeNode { object: node, mode: mode, name: name }
    }
}
