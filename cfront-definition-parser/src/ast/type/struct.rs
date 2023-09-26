use cfront_definition::{token::{TokenType, Token}, Keyword};

use crate::{Parser, ast::AstNode};

#[derive(Debug, PartialEq, Eq, Clone, Copy)] 
pub enum Struct {
    Struct, 
    Union 
}

impl <'a> Parser<'a> for Struct {
    type E = (); 

    fn parse (tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser>::E> { 
        let f = tokens.first().ok_or(())?; 
        let r = &tokens[1..]; 
        let ans = match f.token_type {
            TokenType::Keyword(Keyword::Struct) => (Struct::Struct, r), 
            TokenType::Keyword(Keyword::Union) => (Struct::Union, r), 
            _ => return Err(()), 
        }; 
        return Ok(ans); 
    }
}

/// SpecQualiferList is a list of specifiers and qualifiers. 
pub struct SpecQualiferList <'a> ( pub Vec<AstNode<'a>>); 