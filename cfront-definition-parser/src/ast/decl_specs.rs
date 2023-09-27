use cfront_definition::token::Token;

use crate::Parser;

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DeclSpec <'a> {
    StorageClassSpec(Box<Ast<'a>>) , 
    TypeSpec(Box<Ast<'a>>), 
    TypeQualifier(Box<Ast<'a>>), 
}

impl <'a> Parser<'a> for DeclSpec<'a> {
    type E = ();

    fn parse (tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser>::E> {
        todo!()
    }
} 

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DeclSpecs <'a> ( pub Vec<Ast<'a>> ); 

impl <'a> Parser<'a> for DeclSpecs<'a> {
    type E = ();

    fn parse (tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser>::E> {
        todo!()
    }
} 