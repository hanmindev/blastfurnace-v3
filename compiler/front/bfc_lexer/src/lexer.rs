use crate::error::{Error, ErrorType};
use crate::token::{Token, TokenType};
use bfc_span::span::{BytePos, Span};
use std::iter::Peekable;
use std::str::CharIndices;

struct Lexer<'s> {
    chars: Peekable<CharIndices<'s>>,
    end: usize,
}

impl<'s> Lexer<'s> {
    fn new(input: &'s str) -> Lexer {
        Lexer {
            chars: input.char_indices().peekable(),
            end: input.len(),
        }
    }

    fn try_read_string(&mut self) -> Option<Result<TokenType, ErrorType>> {
        if self.chars.next_if(|(_, c)| *c == '"').is_some() {
            let mut build = String::new();

            for (_, c) in self.chars.by_ref() {
                match c {
                    // TODO: escaping
                    // TODO: multi-line string
                    '"' => {
                        return Some(Ok(TokenType::String(build)));
                    }
                    _ => build.push(c),
                }
            }

            Some(Err(ErrorType::UnclosedString))
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
                    "null" => TokenType::Null,
                    "true" => TokenType::Bool(true),
                    "false" => TokenType::Bool(false),

                    "fn" => TokenType::Fn,
                    "rec" => TokenType::Rec,

                    "if" => TokenType::If,
                    "else" => TokenType::Else,
                    "while" => TokenType::While,
                    "for" => TokenType::For,
                    "return" => TokenType::Return,
                    "break" => TokenType::Break,
                    "continue" => TokenType::Continue,

                    "void" => TokenType::VoidType,
                    "int" => TokenType::IntType,
                    "float" => TokenType::FloatType,
                    "double" => TokenType::DoubleType,
                    "bool" => TokenType::BoolType,
                    "string" => TokenType::StringType,
                    "struct" => TokenType::StructType,

                    "impl" => TokenType::Impl,
                    "let" => TokenType::Let,
                    "const" => TokenType::Const,
                    "inline" => TokenType::Inline,

                    "use" => TokenType::Use,
                    "as" => TokenType::As,
                    "mod" => TokenType::Mod,
                    "pub" => TokenType::Pub,
                    _ => TokenType::Ident(token_str),
                }
            } else {
                return None;
            }
        } else {
            return None;
        })
    }

    fn try_read_number(&mut self, negate: bool) -> Option<Result<TokenType, ErrorType>> {
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

                        Ok(if self.chars.next_if(|(_, c)| *c == 'd').is_some() {
                            TokenType::Double(n)
                        } else {
                            TokenType::Float(n as f32)
                        })
                    }
                    Err(_) => Err(ErrorType::InvalidToken),
                }
            } else {
                match number.parse::<i64>() {
                    Ok(mut n) => {
                        if negate {
                            n = -n;
                        }
                        Ok(if self.chars.next_if(|(_, c)| *c == 'l').is_some() {
                            TokenType::Long(n)
                        } else {
                            TokenType::Int(n as i32)
                        })
                    }
                    Err(_) => Err(ErrorType::InvalidToken),
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

            _ => {
                return None;
            }
        })
    }

    fn try_read_symbol(&mut self) -> Option<Result<TokenType, ErrorType>> {
        if let Some(first) = self.peek_character_token() {
            self.chars.next();

            if let Some(second) = self.peek_character_token() {
                if let Some(merged) = self.match_character_tokens(&first, &second) {
                    self.chars.next();
                    return Some(Ok(merged));
                }
            }

            if first == TokenType::Minus {
                if let Some(number) = self.try_read_number(true) {
                    return Some(number);
                }
            }

            return Some(Ok(first));
        }

        None
    }

    pub fn read_token(&mut self) -> Option<Result<Token, Error>> {
        // skip whitespace and comments
        while let Some((_, c)) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next(); // skip whitespace char
                continue;
            } else {
                break;
            }
        }

        if let Some((idx, _)) = self.chars.peek() {
            let lo = *idx;

            let token_type_result = if let Some(string_literal) = self.try_read_string() {
                string_literal
            } else if let Some(keyword) = self.try_read_keyword() {
                Ok(keyword)
            } else if let Some(number) = self.try_read_number(false) {
                number
            } else if let Some(symbol) = self.try_read_symbol() {
                symbol
            } else {
                return None;
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

            Some(match token_type_result {
                Ok(token_type) => Ok(Token { token_type, span }),
                Err(err) => Err(Error {
                    error_type: err,
                    span,
                }),
            })
        } else {
            None // No more tokens
        }
    }
}
impl Iterator for Lexer<'_> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_token_stream {
        ($lexer: ident, $($token: expr), *) => {
            let mut next = || $lexer.next().unwrap().unwrap();

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
            TokenType::Null,
            TokenType::Bool(true),
            TokenType::Bool(false),
            TokenType::Int(-1234),
            TokenType::Int(0),
            TokenType::Int(1234),
            TokenType::Long(-1234),
            TokenType::Long(0),
            TokenType::Long(1234),
            TokenType::Float(-1234.0),
            TokenType::Float(0.0),
            TokenType::Float(1234.0),
            TokenType::Double(-1234.0),
            TokenType::Double(0.0),
            TokenType::Double(1234.0)
        );

        assert_eq!(
            TokenType::String("hello world".to_string()),
            lexer.next().unwrap().unwrap().token_type
        );
    }

    #[test]
    fn test_unclosed_string() {
        let mut lexer = Lexer::new("\"hello world");

        assert_eq!(
            ErrorType::UnclosedString,
            lexer.next().unwrap().unwrap_err().error_type
        )
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
        let mut lexer = Lexer::new("&& || ++ -- == != <= >= += -= *= /= %=");

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
            TokenType::PercentAssign
        );
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("fn rec if else while for return break continue void int float double bool string struct impl let const inline use as mod pub");

        test_token_stream!(
            lexer,
            TokenType::Fn,
            TokenType::Rec,
            TokenType::If,
            TokenType::Else,
            TokenType::While,
            TokenType::For,
            TokenType::Return,
            TokenType::Break,
            TokenType::Continue,
            TokenType::VoidType,
            TokenType::IntType,
            TokenType::FloatType,
            TokenType::DoubleType,
            TokenType::BoolType,
            TokenType::StringType,
            TokenType::StructType,
            TokenType::Impl,
            TokenType::Let,
            TokenType::Const,
            TokenType::Inline,
            TokenType::Use,
            TokenType::As,
            TokenType::Mod,
            TokenType::Pub
        );
    }
}
