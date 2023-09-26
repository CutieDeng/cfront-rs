use cfront_definition::Keyword;
use cfront_definition::token::{Token, TokenType};

use crate::Parser;

#[derive(Debug, PartialEq, Eq, Clone, Copy)] 
pub enum TypeQualifer {
    Const, 
    Volatile, 
}

impl <'a> Parser<'a> for TypeQualifer {
    type E = (); 
    fn parse (tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), ()> { 
        let first = tokens.first().ok_or(())?; 
        let rest = &tokens[1..]; 
        let ans = match first.token_type {
            TokenType::Keyword(Keyword::Const) => (TypeQualifer::Const, rest),
            TokenType::Keyword(Keyword::Volatile) => (TypeQualifer::Volatile, rest), 
            _ => return Err(()), 
        }; 
        return Ok(ans); 
    }
} 