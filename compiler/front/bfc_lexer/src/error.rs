use bfc_span::span::Span;

#[derive(Debug, PartialEq)]
pub enum ErrorType {
    InvalidToken,
    UnclosedString,
}

#[derive(Debug, PartialEq)]
pub struct Error {
    pub error_type: ErrorType,
    pub span: Span,
}
