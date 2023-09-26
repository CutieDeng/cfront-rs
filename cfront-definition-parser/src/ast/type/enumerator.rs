use cfront_definition::token::Token;

use crate::Parser;

use super::Enumerator;

impl <'a> Parser<'a> for Enumerator<'a> {
    type E = ();
    fn parse (tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), ()> { 
        let f = tokens.first().ok_or(())?;
        let r = &tokens[1..]; 
        let a = (Enumerator(f.clone()), r);
        Ok(a) 
    }
}