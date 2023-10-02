use cfront_definition::token::Token;

use crate::Parser;

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct Decl <'a> {
    pub decl_specs: Box<Ast<'a>>,
    pub init_declarator_list: Option<Box<Ast<'a>>>,
}

impl <'a> Parser<'a> for Decl<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        todo!()
    } 

}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DeclList<'a> (pub Vec<Ast<'a>>); 

impl <'a> Parser<'a> for DeclList<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        todo!()
    }
    
} 