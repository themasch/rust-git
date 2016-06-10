use model::hashable::Hashable;

#[derive(Debug,Clone)]
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


/// this does not care if the content is a blob! use with caution
///
impl From<Vec<u8>> for Blob {
    fn from(data: Vec<u8>) -> Blob {

        // find \0x00
        let mut start = 4;
        for &byte in &data[4..] {
            start += 1;
            if byte == 0 {
                break;
            }
        }

        let mut vec = Vec::new();
        vec.extend_from_slice(&data[start..]);
        Blob::new(vec)
    }
}

impl Hashable for Blob {
    fn get_hash_content(&self) -> Vec<u8> {
        let metadata = self.get_metadata();
        let content = self.get_content();
        let mut vec = Vec::with_capacity(metadata.len() + content.len() + 1);
        vec.extend(metadata);
        vec.push(0x00);
        vec.extend(content);

        vec
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
    assert_eq!(String::from("blob 4").into_bytes(), x.get_metadata());
}

#[test]
fn hash_blob() {
    let test = Blob::new(String::from("The quick brown fox jumps over the lazy dog").into_bytes());
    let expected = vec![255, 59, 182, 57, 72, 180, 178, 71, 150, 210, 172, 210, 89, 145, 95, 42, 157, 151, 38, 56];
    let result = test.hash();
    assert_eq!(result.len(),20);
    assert_eq!(result, expected);
}


#[test]
fn test_blob_from() {
    let mut vec = Vec::new();
    vec.extend_from_slice(include_bytes!("../../fixtures/blob.git-file.unpacked"));
    let blob = Blob::from(vec);
    let expect = vec![255, 235, 158, 73, 149, 185, 14, 166, 198, 50, 151, 41, 87, 111, 6, 240, 88, 208, 228, 187];
    let result = blob.hash();
    assert_eq!(result.len(),20);
    assert_eq!(result, expect);
}
