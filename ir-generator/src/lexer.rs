use crate::error::CompilerResult;
use crate::error::UnexpectedCharacterError;
use crate::ranged::{Ranged, ToRanged};
use crate::token::Token;
use std::iter::{Enumerate, Peekable};
use std::str::Chars;

pub struct Lexer<'a> {
    src: Peekable<Enumerate<Chars<'a>>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer {
            src: src.chars().enumerate().peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.src.next_if(|(_, c)| c.is_whitespace()).is_some() {}
    }

    fn parse_number(&mut self) -> Option<CompilerResult<Ranged<Token>>> {
        let mut result = String::new();
        let mut encountered_dot = false;

        let start = self.src.peek()?.0;
        let mut end = start;

        while let Some((idx, c)) = self
            .src
            .next_if(|(_, c)| c.is_numeric() || *c == '_' || (*c == '.' && !encountered_dot))
        {
            result.push(c);
            end = idx;
            if c == '.' {
                encountered_dot = true;
            }
        }

        Some(Ok(Token::Number(result).ranged(start..=end)))
    }

    fn parse_ident(&mut self) -> Option<CompilerResult<Ranged<Token>>> {
        let mut result = String::new();

        let start = self.src.peek()?.0;
        let mut end = start;

        while let Some((idx, c)) = self.src.next_if(|(_, c)| c.is_alphanumeric() || *c == '_') {
            result.push(c);
            end = idx;
        }

        let token = match result.as_str() {
            "fn" => Token::Fn,
            "const" => Token::Const,
            "let" => Token::Let,
            "in" => Token::In,
            "for" => Token::For,
            "downto" => Token::Downto,
            "loop" => Token::Loop,
            "f16" => Token::F16,
            "i16" => Token::I16,
            "u16" => Token::U16,
            "as" => Token::As,
            "alias" => Token::Alias,
            _ => Token::Ident(result),
        };

        Some(Ok(token.ranged(start..=end)))
    }

    fn parse_operator(&mut self) -> Option<CompilerResult<Ranged<Token>>> {
        let (idx, next) = self.src.next()?;

        Some(match next {
            '(' => Ok(Token::LParen.ranged(idx..=idx)),
            ')' => Ok(Token::RParen.ranged(idx..=idx)),
            '{' => Ok(Token::LBrace.ranged(idx..=idx)),
            '}' => Ok(Token::RBrace.ranged(idx..=idx)),
            ';' => Ok(Token::Semicolon.ranged(idx..=idx)),
            ':' => match self.src.next_if(|(_, c)| *c == ':') {
                Some(_) => Ok(Token::Scope.ranged(idx..=idx)),
                None => Ok(Token::Colon.ranged(idx..=idx)),
            },
            ',' => Ok(Token::Comma.ranged(idx..=idx)),
            '=' => Ok(Token::Assign.ranged(idx..=idx)),
            '+' => Ok(Token::Plus.ranged(idx..=idx)),
            '!' => Ok(Token::Bang.ranged(idx..=idx)),
            '-' => match self.src.next_if(|(_, c)| *c == '>') {
                Some(_) => Ok(Token::Arrow.ranged(idx..=idx)),
                None => Ok(Token::Minus.ranged(idx..=idx)),
            },
            '*' => Ok(Token::Asterisk.ranged(idx..=idx)),
            '/' => Ok(Token::Slash.ranged(idx..=idx)),
            '&' => Ok(Token::Ampersand.ranged(idx..=idx)),
            _ => Err(UnexpectedCharacterError {
                character: next,
                idx,
            }
            .into()),
        })
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = CompilerResult<Ranged<Token>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        match self.src.peek()? {
            (_, c) if c.is_numeric() => self.parse_number(),
            (_, c) if c.is_alphabetic() || *c == '_' => self.parse_ident(),
            _ => self.parse_operator(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CompilerError;

    #[test]
    fn test_tokenization() {
        let code = "let x = 10 + 4;";
        let mut lexer = Lexer::new(code);
        assert_eq!(lexer.next(), Some(Ok((0..=2, Token::Let))));
        assert_eq!(
            lexer.next(),
            Some(Ok((4..=4, Token::Ident(String::from("x")))))
        );
        assert_eq!(lexer.next(), Some(Ok((6..=6, Token::Assign))));
        assert_eq!(
            lexer.next(),
            Some(Ok((8..=9, Token::Number(String::from("10")))))
        );
        assert_eq!(lexer.next(), Some(Ok((11..=11, Token::Plus))));
        assert_eq!(
            lexer.next(),
            Some(Ok((13..=13, Token::Number(String::from("4")))))
        );
        assert_eq!(lexer.next(), Some(Ok((14..=14, Token::Semicolon))));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_unexpected_character() {
        let code = "let x = 10 § 4;";
        let mut lexer = Lexer::new(code);
        for _ in 0..4 {
            let _ = lexer.next();
        }
        assert_eq!(
            lexer.next(),
            Some(Err(CompilerError::UnexpectedCharacterError(
                UnexpectedCharacterError {
                    character: '§',
                    idx: 11,
                }
            )))
        );
    }
}
