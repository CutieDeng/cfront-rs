use crate::Parser;

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DirectAbstractDeclarator <'a> {
    AbstractDeclarator(Box<Ast<'a>>), 
    PostBracket(Option<Box<Ast<'a>>>),
    PostParenthesis(Option<Box<Ast<'a>>>),
}

impl <'a> Parser<'a> for DirectAbstractDeclarator<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [cfront_definition::token::Token<'a>]) -> Result<(Self, &'a [cfront_definition::token::Token<'a>]), <Self as Parser<'a>>::E> {
        todo!()
    }
}