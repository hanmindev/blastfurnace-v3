use crate::error::TokenError;
use crate::token::{Token, TokenType};
use bfc_span::span::{BytePos, Span};
use std::iter::Peekable;
use std::str::CharIndices;

pub struct Lexer<'s> {
    chars: Peekable<CharIndices<'s>>,
    end: usize,
}

impl<'s> Lexer<'s> {
    pub fn new(input: &'s str) -> Lexer {
        Lexer {
            chars: input.char_indices().peekable(),
            end: input.len(),
        }
    }

    fn get_eof_token(&self) -> Token {
        Token {
            token_type: TokenType::Eof,
            span: Span {
                lo: BytePos(self.end as u32),
                hi: BytePos(self.end as u32),
            },
        }
    }

    fn try_read_string(&mut self) -> Option<TokenType> {
        if self.chars.next_if(|(_, c)| *c == '"').is_some() {
            let mut build = String::new();

            for (_, c) in self.chars.by_ref() {
                match c {
                    // TODO: escaping
                    // TODO: multi-line string
                    '"' => {
                        return Some(TokenType::LitStr(build));
                    }
                    _ => build.push(c),
                }
            }

            Some(TokenType::Unknown(TokenError::UnclosedString))
        } else {
            None
        }
    }

    fn try_read_keyword(&mut self) -> Option<TokenType> {
        Some(if let Some((_, c)) = self.chars.peek() {
            if c.is_alphabetic() || *c == '_' {
                // prevent starting with number
                let mut token_str = String::new();
                while let Some((_, c)) = self.chars.peek() {
                    if c.is_alphanumeric() || *c == '_' {
                        token_str.push(*c);
                        self.chars.next();
                    } else {
                        break;
                    }
                }

                if token_str.is_empty() {
                    return None;
                }

                match token_str.as_str() {
                    "null" => TokenType::LitNull,
                    "true" => TokenType::LitBool(true),
                    "false" => TokenType::LitBool(false),

                    "fn" => TokenType::KwFn,
                    "rec" => TokenType::KwRec,

                    "if" => TokenType::KwIf,
                    "else" => TokenType::KwElse,
                    "while" => TokenType::KwWhile,
                    "for" => TokenType::KwFor,
                    "return" => TokenType::KwReturn,
                    "break" => TokenType::KwBreak,
                    "continue" => TokenType::KwContinue,

                    "void" => TokenType::KwVoid,
                    "i32" => TokenType::KwI32,
                    "i64" => TokenType::KwI64,
                    "f32" => TokenType::KwF32,
                    "f64" => TokenType::KwF64,
                    "bool" => TokenType::KwBool,
                    "string" => TokenType::KwStr,
                    "struct" => TokenType::KwStruct,

                    "impl" => TokenType::KwImpl,
                    "let" => TokenType::KwLet,
                    "const" => TokenType::KwConst,
                    "inline" => TokenType::KwInline,

                    "use" => TokenType::KwUse,
                    "as" => TokenType::KwAs,
                    "mod" => TokenType::KwMod,
                    "pub" => TokenType::KwPub,
                    _ => TokenType::Ident(token_str),
                }
            } else {
                return None;
            }
        } else {
            return None;
        })
    }

    fn try_read_number(&mut self, negate: bool) -> Option<TokenType> {
        Some(if let Some((_, c)) = self.chars.peek() {
            if !c.is_ascii_digit() {
                return None;
            }

            let mut number = String::new();
            let mut is_decimal = false;

            while let Some((_, c)) = self
                .chars
                .next_if(|(_, c)| c.is_ascii_digit() || (*c == '.' && !is_decimal))
            {
                number.push(c);
                if c == '.' {
                    is_decimal = true;
                }
            }

            if is_decimal {
                match number.parse::<f64>() {
                    Ok(mut n) => {
                        if negate {
                            n = -n;
                        }

                        if self.chars.next_if(|(_, c)| *c == 'd').is_some() {
                            TokenType::LitF64(n)
                        } else {
                            TokenType::LitF32(n as f32)
                        }
                    }
                    Err(_) => TokenType::Unknown(TokenError::InvalidToken),
                }
            } else {
                match number.parse::<i64>() {
                    Ok(mut n) => {
                        if negate {
                            n = -n;
                        }
                        if self.chars.next_if(|(_, c)| *c == 'l').is_some() {
                            TokenType::LitI64(n)
                        } else {
                            TokenType::LitI32(n as i32)
                        }
                    }
                    Err(_) => TokenType::Unknown(TokenError::InvalidToken),
                }
            }
        } else {
            return None;
        })
    }

    fn peek_character_token(&mut self) -> Option<TokenType> {
        match self.chars.peek() {
            None => None,
            Some((_, c)) => Some(match c {
                '+' => TokenType::Plus,
                '-' => TokenType::Minus,
                '*' => TokenType::Star,
                '/' => TokenType::Slash,
                '%' => TokenType::Percent,
                '!' => TokenType::Exclamation,
                '&' => TokenType::Ampersand,
                '|' => TokenType::Pipe,
                '=' => TokenType::Assign,
                ',' => TokenType::Comma,
                ';' => TokenType::Semicolon,
                ':' => TokenType::Colon,
                '.' => TokenType::Dot,
                '(' => TokenType::LParen,
                ')' => TokenType::RParen,
                '{' => TokenType::LBrace,
                '}' => TokenType::RBrace,
                '[' => TokenType::LBracket,
                ']' => TokenType::RBracket,
                '<' => TokenType::LAngle,
                '>' => TokenType::RAngle,
                _ => {
                    return None;
                }
            }),
        }
    }

    fn match_character_tokens(&self, first: &TokenType, second: &TokenType) -> Option<TokenType> {
        Some(match (first, second) {
            (TokenType::LAngle, TokenType::Assign) => TokenType::Leq, // <=
            (TokenType::RAngle, TokenType::Assign) => TokenType::Geq, // >=

            (TokenType::Assign, TokenType::Assign) => TokenType::Equal, // ==
            (TokenType::Exclamation, TokenType::Assign) => TokenType::NotEqual, // !=

            (TokenType::Plus, TokenType::Assign) => TokenType::PlusAssign, // +=
            (TokenType::Minus, TokenType::Assign) => TokenType::MinusAssign, // -=
            (TokenType::Star, TokenType::Assign) => TokenType::StarAssign, // *=
            (TokenType::Slash, TokenType::Assign) => TokenType::SlashAssign, // /=
            (TokenType::Percent, TokenType::Assign) => TokenType::PercentAssign, // %=

            (TokenType::Ampersand, TokenType::Ampersand) => TokenType::And, // &&
            (TokenType::Pipe, TokenType::Pipe) => TokenType::Or,            // ||
            (TokenType::Plus, TokenType::Plus) => TokenType::PlusPlus,      // ++
            (TokenType::Minus, TokenType::Minus) => TokenType::MinusMinus,  // --

            (TokenType::Minus, TokenType::RAngle) => TokenType::Arrow, // ->
            (TokenType::Colon, TokenType::Colon) => TokenType::DoubleColon, // ::

            _ => {
                return None;
            }
        })
    }

    fn try_read_symbol(&mut self) -> Option<TokenType> {
        if let Some(first) = self.peek_character_token() {
            self.chars.next();

            if let Some(second) = self.peek_character_token() {
                if let Some(merged) = self.match_character_tokens(&first, &second) {
                    self.chars.next();
                    return Some(merged);
                }
            }

            if first == TokenType::Minus {
                if let Some(number) = self.try_read_number(true) {
                    return Some(number);
                }
            }

            return Some(first);
        }

        None
    }

    pub fn read_token(&mut self) -> Token {
        // skip whitespace and comments
        while let Some((_, c)) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next(); // skip whitespace char
                continue;
            } else {
                break;
            }
        }

        if let Some((idx, ch)) = self.chars.peek() {
            let lo = *idx;
            let ch = *ch;

            let token_type = if let Some(string_literal) = self.try_read_string() {
                string_literal
            } else if let Some(keyword) = self.try_read_keyword() {
                keyword
            } else if let Some(number) = self.try_read_number(false) {
                number
            } else if let Some(symbol) = self.try_read_symbol() {
                symbol
            } else {
                TokenType::Unknown(TokenError::UnknownCharacter(ch))
            };

            // create span
            let hi = if let Some((n, _)) = self.chars.peek() {
                lo + *n
            } else {
                self.end
            };

            let span = Span {
                lo: BytePos(lo as u32),
                hi: BytePos(hi as u32),
            };

            Token { token_type, span }
        } else {
            self.get_eof_token()
        }
    }
}
impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.read_token())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_token_stream {
        ($lexer: ident, $($token: expr), *) => {
            let mut next = || $lexer.next().unwrap();

            $(
                assert_eq!($token, next().token_type);
            ) *
        };
    }

    #[test]
    fn test_literal_tokens() {
        let mut lexer = Lexer::new(
            "null true false -1234 0 1234 -1234l 0l 1234l -1234.0 0.0 1234.0 -1234.0d 0.0d 1234.0d \"hello world\"",
        );
        test_token_stream!(
            lexer,
            TokenType::LitNull,
            TokenType::LitBool(true),
            TokenType::LitBool(false),
            TokenType::LitI32(-1234),
            TokenType::LitI32(0),
            TokenType::LitI32(1234),
            TokenType::LitI64(-1234),
            TokenType::LitI64(0),
            TokenType::LitI64(1234),
            TokenType::LitF32(-1234.0),
            TokenType::LitF32(0.0),
            TokenType::LitF32(1234.0),
            TokenType::LitF64(-1234.0),
            TokenType::LitF64(0.0),
            TokenType::LitF64(1234.0),
            TokenType::LitStr("hello world".to_string())
        );
    }

    #[test]
    fn test_unclosed_string() {
        let mut lexer = Lexer::new("\"hello world");

        test_token_stream!(lexer, TokenType::Unknown(TokenError::UnclosedString));
    }

    #[test]
    fn test_singletons() {
        let mut lexer = Lexer::new("= + - * / % ! & | , ; : . ( ) { } [ ] < >");

        test_token_stream!(
            lexer,
            TokenType::Assign,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Star,
            TokenType::Slash,
            TokenType::Percent,
            TokenType::Exclamation,
            TokenType::Ampersand,
            TokenType::Pipe,
            TokenType::Comma,
            TokenType::Semicolon,
            TokenType::Colon,
            TokenType::Dot,
            TokenType::LParen,
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::RBrace,
            TokenType::LBracket,
            TokenType::RBracket,
            TokenType::LAngle,
            TokenType::RAngle
        );
    }

    #[test]
    fn test_merged() {
        let mut lexer = Lexer::new("&& || ++ -- == != <= >= += -= *= /= %= -> ::");

        test_token_stream!(
            lexer,
            TokenType::And,
            TokenType::Or,
            TokenType::PlusPlus,
            TokenType::MinusMinus,
            TokenType::Equal,
            TokenType::NotEqual,
            TokenType::Leq,
            TokenType::Geq,
            TokenType::PlusAssign,
            TokenType::MinusAssign,
            TokenType::StarAssign,
            TokenType::SlashAssign,
            TokenType::PercentAssign,
            TokenType::Arrow,
            TokenType::DoubleColon
        );
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("fn rec if else while for return break continue void i32 i64 f32 f64 bool string struct impl let const inline use as mod pub");

        test_token_stream!(
            lexer,
            TokenType::KwFn,
            TokenType::KwRec,
            TokenType::KwIf,
            TokenType::KwElse,
            TokenType::KwWhile,
            TokenType::KwFor,
            TokenType::KwReturn,
            TokenType::KwBreak,
            TokenType::KwContinue,
            TokenType::KwVoid,
            TokenType::KwI32,
            TokenType::KwI64,
            TokenType::KwF32,
            TokenType::KwF64,
            TokenType::KwBool,
            TokenType::KwStr,
            TokenType::KwStruct,
            TokenType::KwImpl,
            TokenType::KwLet,
            TokenType::KwConst,
            TokenType::KwInline,
            TokenType::KwUse,
            TokenType::KwAs,
            TokenType::KwMod,
            TokenType::KwPub
        );
    }

    #[test]
    fn test_eof() {
        let mut lexer = Lexer::new("fn");

        test_token_stream!(
            lexer,
            TokenType::KwFn,
            TokenType::Eof,
            TokenType::Eof,
            TokenType::Eof,
            TokenType::Eof
        );
    }
}
