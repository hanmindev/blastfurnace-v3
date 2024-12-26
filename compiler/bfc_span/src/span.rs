#[derive(Debug, PartialEq)]
pub struct BytePos(pub u32);
#[derive(Debug, PartialEq)]
pub struct Span {
    pub lo: BytePos,
    pub hi: BytePos,
}
