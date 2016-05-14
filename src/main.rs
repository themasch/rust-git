mod model;

use model::Object;
use model::Blob;
use model::Tree;
use model::hashable::Hashable;
use model::tree_node::TreeNode;
use std::str;

use std::io;
use std::io::prelude::*;
use std::fs::File;

fn read_object<'a>(data: Vec<u8>) -> Result<Object<'a>, String> {
    match str::from_utf8(&data[0..3]).unwrap() {
        "blob" => {
            Ok(Object::Blob(Blob::new(String::from("test").into_bytes())))
        },
        _ => {
            Err(String::from("unknown object"))
        }
    }
}

fn main() {

    let mut f = File::open("./fixtures/blob.git-file").unwrap();
    let mut buffer = vec![0; 10];
    // read the whole file
    f.read_to_end(&mut buffer);

    println!("{:?}", read_object(buffer));
}
