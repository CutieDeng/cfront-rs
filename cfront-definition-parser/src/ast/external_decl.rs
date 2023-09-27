use cfront_definition::token::Token;

use crate::Parser;

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExternalDecl <'a> {
    FunctionDefinition(Box<Ast<'a>>),
    Decl(Box<Ast<'a>>),
}

impl <'a> Parser<'a> for ExternalDecl<'a> {
    type E = ();

    fn parse (tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser>::E> {
        todo!()
    }
} 
