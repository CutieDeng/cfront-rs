use crate::Parser;

use super::Ast;

pub struct ParamList <'a> { 
    pub params: Vec<Ast<'a>>, 
    pub trailing_comma: bool, 
} 

impl <'a> Parser<'a> for ParamList<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [cfront_definition::token::Token<'a>]) -> Result<(Self, &'a [cfront_definition::token::Token<'a>]), <Self as Parser<'a>>::E> {
        
        todo!()
    }
}