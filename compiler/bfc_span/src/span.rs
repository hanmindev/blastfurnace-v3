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

pub fn join_spans<'a, I>(spans: I) -> Span
where
    I: Iterator<Item = &'a Span>,
{
    let mut lo = 0;
    let mut hi = 0;

    for span in spans {
        lo = lo.min(span.lo.0);
        hi = hi.max(span.hi.0);
    }

    Span {
        lo: BytePos(lo),
        hi: BytePos(hi),
    }
}
