use cfront_definition::token::Token;

use crate::Parser;

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EnumSpec <'a> {
    pub r#enum: Token<'a>, 
    pub enumerator_list: Option<Box<Ast<'a>>>, 
}

impl <'a> Parser<'a> for EnumSpec<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        todo!()
    }
    
} 