use cfront_definition::token::{Token, TokenType};

use crate::Parser;

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct StructOrUnionSpec <'a> {
    pub r#struct: Token<'a>, 
    pub identifier: Option<Box<Ast<'a>>>, 
    pub struct_decl_list: Option<Box<Ast<'a>>>, 
}

impl <'a> Parser<'a> for StructOrUnionSpec<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        todo!()
    }
    
} 