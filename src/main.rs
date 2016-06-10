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


fn read_object(data: Vec<u8>) -> Result<Object, String> {
    match str::from_utf8(&data[0..4]).unwrap() {
        "blob" => {
            let blub = Blob::from(data);

            println!("{:?}", str::from_utf8(&blub.get_content()));
            println!("{:?}", blub.hash());

            Ok(Object::from(blub))
        },
        _ => {
            Err(String::from("unknown object"))
        }
    }
}

fn main() {

println!("reading ./fixtures/blob.git-file.unpacked", );
    let mut f = File::open("./fixtures/blob.git-file.unpacked").unwrap();
    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer);

    println!("{:?}", read_object(buffer));
}
