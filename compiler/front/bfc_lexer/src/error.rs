#[derive(Debug, Clone, PartialEq)]
pub enum TokenError {
    InvalidToken,
    UnclosedString,
}
