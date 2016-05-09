mod object;
mod blob;
mod tree_node;
mod tree;

use object::Object;
use blob::Blob;
use tree::Tree;
use tree_node::TreeNode;
use std::io::{self, Write};

fn main() {
    //println!("Hello, world!");
    let testa = Blob::new(String::from("The quick brown fox jumps over the lazy dog").into_bytes());
    let testb = Blob::new(String::from("The quick brown fox flys over the lazy dog").into_bytes());
    let nodea = TreeNode::new(&testa, 0o100755, String::from("TestA"));
    let nodeb = TreeNode::new(&testb, 0o100644, String::from("Testb"));
    let mut tree = Tree::new();
    tree.add(&nodea);
    tree.add(&nodeb);

    match io::stdout().write(&tree.get_hash_content()) {
        Err(err) => {
            println!("{:?}", err);
        },
        Ok(size) => {
            io::stdout().flush();
        }
    }
    //println!("{:?}", tree.hash());

}
