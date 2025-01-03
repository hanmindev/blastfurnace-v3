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
    LitNull,        // null
    LitBool(bool),  // true / false
    LitI32(i32),    // -1, 0, 1, etc
    LitI64(i64),    // -1l, 0l, 1l, etc
    LitF32(f32),    // -1.0, 0.0, 1.0, etc
    LitF64(f64),    // -1.0d, 0.0d, 1.0d, etc
    LitStr(String), // "hello"
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
    KwConst,    // const
    KwVoid,     // void
    KwI32,      // i32
    KwI64,      // i64
    KwF32,      // f32
    KwF64,      // f64
    KwBool,     // bool
    KwStr,      // str
    KwStruct,   // struct
    KwImpl,     // impl
    KwLet,      // let
    KwFn,       // fn
    KwRec,      // rec
    KwInline,   // inline
    KwIf,       // if
    KwElse,     // else
    KwWhile,    // while
    KwFor,      // for
    KwReturn,   // return
    KwBreak,    // break
    KwContinue, // continue
    KwUse,      // use
    KwAs,       // as
    KwMod,      // mod
    KwPub,      // pub
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
        assert!(is_same(&TokenType::LitNull, TokenKind::LitNull));
        assert!(is_same(&TokenType::KwFn, TokenKind::KwFn));
        assert!(is_same(&TokenType::Eof, TokenKind::Eof));
    }
    #[test]
    fn test_internal_comparison() {
        assert!(is_same(
            &TokenType::Ident("hello".to_string()),
            TokenKind::Ident
        ));
        assert!(is_same(&TokenType::LitI32(1234), TokenKind::LitI32));
        assert!(is_same(
            &TokenType::LitStr("hello".to_string()),
            TokenKind::LitStr
        ));
    }
}
