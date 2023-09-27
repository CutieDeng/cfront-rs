use cfront_definition::{token::{Token, TokenType}, Keyword};

use crate::Parser;

#[derive(Debug, PartialEq, Eq, Clone, )]
pub struct StorageClassSpec <'a> (pub Token<'a> );

impl <'a> Parser<'a> for StorageClassSpec <'a> {

    type E = ();

    fn parse (tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser>::E> {
        let first = tokens.first().ok_or(())?; 
        let rst = match first.token_type {
            | TokenType::Keyword(Keyword::Auto) 
            | TokenType::Keyword(Keyword::Register) 
            | TokenType::Keyword(Keyword::Static) 
            | TokenType::Keyword(Keyword::Extern) 
            | TokenType::Keyword(Keyword::Typedef) 
            => (Self(first.clone()), &tokens[1..]),
            _ => return Err(()), 
        }; 
        return Ok(rst) 
    }
    
} 