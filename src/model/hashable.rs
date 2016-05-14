extern crate sha1;
use std::fmt::Write;
use std::fmt;

pub trait Hashable {
    /// returns the complete object content, including the metadata
    fn get_hash_content(&self) -> Vec<u8>;

    /// returns the hash for this object
    fn hash(&self) -> Vec<u8> {
        let mut sha = sha1::Sha1::new();
        sha.update(&self.get_hash_content());
        sha.digest()
    }
}

impl fmt::Debug for Hashable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for byte in self.hash() {
            write!(&mut s, "{:X} ", byte).unwrap();
        }

        write!(f, "Hashable {{ hash: {} }}", s)
   }
}

#[test]
pub fn test_mock() {
    struct HashableMock {
        text: String
    }

    impl Hashable for HashableMock {
        fn get_hash_content(&self) -> Vec<u8> {
            self.text.clone().into_bytes()
        }
    }

    let test = HashableMock { text: String::from("The quick brown fox jumps over the lazy dog") };
    let expected = vec![47, 212, 225, 198, 122, 45, 40, 252, 237, 132, 158, 225, 187, 118, 231, 57, 27, 147, 235, 18];
    let result = test.hash();
    assert_eq!(result.len(), expected.len());
    assert_eq!(result, expected);
}
