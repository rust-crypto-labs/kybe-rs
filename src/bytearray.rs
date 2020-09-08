#[derive(Debug)]
pub struct ByteArray {}

impl ByteArray {
    pub fn random() -> Self {
        unimplemented!()
    }

    pub fn append(&self, _other: &Self) -> Self {
        unimplemented!()
    }
}

impl PartialEq for ByteArray {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl Eq for ByteArray {}
