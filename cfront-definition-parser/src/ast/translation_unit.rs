use cfront_definition::token::Token;

use crate::Parser;

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct TranslationUnit <'a> (pub Vec<Ast<'a>>); 

impl <'a> Parser<'a> for TranslationUnit<'a> {
    type E = (); 

    fn parse (tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser>::E> {
        todo!()
    }
}
