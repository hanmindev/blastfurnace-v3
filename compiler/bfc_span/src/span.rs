#[derive(Debug, PartialEq, Clone)]
pub struct BytePos(pub u32);
#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub lo: BytePos,
    pub hi: BytePos,
}
