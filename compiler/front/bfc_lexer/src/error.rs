#[derive(Debug, Clone, PartialEq)]
pub enum TokenError {
    UnknownCharacter(char),
    InvalidToken,
    UnclosedString,
}
