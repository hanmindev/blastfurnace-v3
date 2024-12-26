#[derive(Debug, PartialEq, Clone)]
pub struct BytePos(pub u32);
#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub lo: BytePos,
    pub hi: BytePos,
}

pub const DUMMY_SPAN: Span = Span {
    lo: BytePos(0),
    hi: BytePos(0),
};
