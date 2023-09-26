use cfront_definition::token::Token;

use crate::Parser;

use super::compound::Expression;

pub struct Enumerator <'a> (pub Token<'a>, pub Option<Box<Expression<'a>>>);

impl <'a> Parser<'a> for Enumerator <'a> {
    type E = ();
    fn parse (tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), ()> { 
        todo!()
    } 
}