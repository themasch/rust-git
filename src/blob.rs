use object::Object;

pub struct Blob {
    content: Vec<u8>
}

impl Blob {
    pub fn new(content: Vec<u8>) -> Blob {
        Blob { content: content }
    }

    pub fn content_length(&self) -> usize {
        self.content.len()
    }

    pub fn get_content(&self) -> Vec<u8> {
        self.content.clone()
    }

    pub fn get_metadata(&self) -> Vec<u8> {
        format!("blob {}", self.content_length()).into_bytes()
    }
}

impl Object for Blob {
    fn get_hash_content(&self) -> Vec<u8> {
        self.get_content()
    }
}

#[test]
fn create_blob() {
    let x = Blob::new(String::from("asdf").into_bytes());
    assert_eq!(4, x.content_length());
}

#[test]
fn test_metadata() {
    let x = Blob::new(String::from("asdf").into_bytes());
    assert_eq!("blob 4", x.get_metadata());
}

#[test]
fn hash_blob() {
    let test = Blob::new(String::from("The quick brown fox jumps over the lazy dog").into_bytes());
    let expected = vec![47, 212, 225, 198, 122, 45, 40, 252, 237, 132, 158, 225, 187, 118, 231, 57, 27, 147, 235, 18];
    let result = test.hash();
    assert_eq!(result.len(),20);
    assert_eq!(result, expected);
}
