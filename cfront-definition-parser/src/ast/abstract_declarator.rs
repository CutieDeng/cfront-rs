use cfront_definition::token::Token;

use crate::Parser;

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone, )] 
pub struct AbstractDeclarator <'a> {
    pub pointer: Option<Box<Ast<'a>>>, 
    pub direct_abstract_declarator: Option<Box<Ast<'a>>>, 
}

impl <'a> Parser<'a> for AbstractDeclarator<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        todo!()
    } 

} 