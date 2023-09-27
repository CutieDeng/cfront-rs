use cfront_definition::token::Token;

use crate::Parser;

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct FunctionDefinition <'a> {
    pub decl_specs: Option<Box<Ast<'a>>>, 
    pub declarator: Box<Ast<'a>>, 
    pub decl_list: Option<Box<Ast<'a>>>, 
    pub compound_statement: Box<Ast<'a>>, 
}

impl <'a> Parser<'a> for FunctionDefinition<'a> {
    type E = (); 

    fn parse (tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser>::E> {
        todo!()
    }
}
