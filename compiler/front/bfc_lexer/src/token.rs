use bfc_span::span::Span;
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
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
    Comma,     // ,
    Semicolon, // ;
    Colon,     // :
    Dot,       // .
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    LBracket,  // [
    RBracket,  // ]
    LAngle,    // <
    RAngle,    // >

    Arrow, // ->

    // Keywords
    Const, // const

    VoidType,   // void
    IntType,    // i32
    FloatType,  // f32
    DoubleType, // f64
    BoolType,   // bool
    StringType, // str
    StructType, // struct

    Impl, // impl

    Let, // let

    Fn,     // fn
    Rec,    // rec
    Inline, // inline

    If,    // if
    Else,  // else
    While, // while
    For,   // for

    Return,   // return
    Break,    // break
    Continue, // continue

    Use, // use
    As,  // as
    Mod, // mod
    Pub, // pub

    // Misc
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}
