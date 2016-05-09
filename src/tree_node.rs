use object::Object;

pub struct TreeNode<'a> {
    pub mode: u16,
    pub object: &'a Object,
    pub name: String
}

impl<'a>  TreeNode<'a> {
    pub fn get_mode_octal(&self) -> Vec<u8> {
        vec![
            ((self.mode / 0o100000) as u16 % 8) as u8,
            ((self.mode / 0o010000) as u16 % 8) as u8,
            ((self.mode / 0o001000) as u16 % 8) as u8,
            ((self.mode / 0o000100) as u16 % 8) as u8,
            ((self.mode / 0o000010) as u16 % 8) as u8,
            ((self.mode / 0o000001) as u16 % 8) as u8,
            (self.mode % 8) as u8,
        ]
    }

    pub fn new(node: &'a Object, mode: u16, name: String) -> TreeNode<'a> {
        TreeNode { object: node, mode: mode, name: name }
    }
}
