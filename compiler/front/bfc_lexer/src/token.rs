use crate::error::TokenError;
use bfc_span::span::Span;

pub fn is_same(ty: &TokenType, kind: TokenKind) -> bool {
    // we can do this since we fully control the definition of both of the enums
    let ty_id = unsafe { *<*const _>::from(ty).cast::<u8>() };
    let kind_id = unsafe { *<*const _>::from(&kind).cast::<u8>() };

    ty_id == kind_id
}

macro_rules! token_macro {
    ($($variant:ident $(($internal:ty))?),*) => {
        #[derive(Debug, Clone, PartialEq)]
        #[repr(u8)]
        pub enum TokenType {
            $(
            $variant$(($internal))?
            ),*
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        #[repr(u8)]
        pub enum TokenKind {
            $($variant),*
        }
    };
}

token_macro!(
    Ident(String), // ident
    // Literals
    Null,           // null
    Bool(bool),     // true / false
    Int(i32),       // -1, 0, 1, etc
    Long(i64),      // -1l, 0l, 1l, etc
    Float(f32),     // -1.0, 0.0, 1.0, etc
    Double(f64),    // -1.0d, 0.0d, 1.0d, etc
    String(String), // "hello"
    // Operators
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    Percent,     // %
    Exclamation, // !
    Ampersand,   // &
    Pipe,        // |
    And,         // &&
    Or,          // ||
    PlusPlus,    // ++
    MinusMinus,  // --
    // Comparison
    Equal,    // ==
    NotEqual, // !=
    Leq,      // <=
    Geq,      // >=
    // Assignment
    Assign,        // =
    PlusAssign,    // +=
    MinusAssign,   // -=
    StarAssign,    // *=
    SlashAssign,   // /=
    PercentAssign, // %=
    // Other Symbols
    Comma,       // ,
    Semicolon,   // ;
    Colon,       // :
    Dot,         // .
    LParen,      // (
    RParen,      // )
    LBrace,      // {
    RBrace,      // }
    LBracket,    // [
    RBracket,    // ]
    LAngle,      // <
    RAngle,      // >
    Arrow,       // ->
    DoubleColon, // ::
    // Keywords
    Const,      // const
    VoidType,   // void
    IntType,    // i32
    FloatType,  // f32
    DoubleType, // f64
    BoolType,   // bool
    StringType, // str
    StructType, // struct
    Impl,       // impl
    Let,        // let
    Fn,         // fn
    Rec,        // rec
    Inline,     // inline
    If,         // if
    Else,       // else
    While,      // while
    For,        // for
    Return,     // return
    Break,      // break
    Continue,   // continue
    Use,        // use
    As,         // as
    Mod,        // mod
    Pub,        // pub
    // Misc
    Eof,
    Unknown(TokenError)
);

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}

#[cfg(test)]
mod test {
    use crate::token::{is_same, TokenKind, TokenType};

    #[test]
    fn test_simple_comparison() {
        assert!(is_same(&TokenType::Null, TokenKind::Null));
        assert!(is_same(&TokenType::Fn, TokenKind::Fn));
        assert!(is_same(&TokenType::Eof, TokenKind::Eof));
    }
    #[test]
    fn test_internal_comparison() {
        assert!(is_same(
            &TokenType::Ident("hello".to_string()),
            TokenKind::Ident
        ));
        assert!(is_same(
            &TokenType::Int(1234),
            TokenKind::Int
        ));
        assert!(is_same(
            &TokenType::String("hello".to_string()),
            TokenKind::String
        ));
    }
}
