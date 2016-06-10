use model::object::Object;

#[derive(Debug)]
pub struct TreeNode {
    pub mode: u16,
    pub object: Box<Object>,
    pub name: String
}

impl TreeNode {
    pub fn new(node: Box<Object>, mode: u16, name: String) -> TreeNode {
        TreeNode { object: node, mode: mode, name: name }
    }
}

#[cfg(test)]
use model::blob::Blob;

#[test]
fn test_create_tree_node() {
    let x = Object::from(Blob::new(String::from("asdf").into_bytes()));
    let node = TreeNode::new(Box::new(x), 0x1234, String::from("filena.me"));

    match *node.object {
        Object::Blob(x) => {
            assert_eq!(x.content_length(), 4);
        },
        _ => assert!(false)
    }
}
