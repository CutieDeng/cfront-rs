use crate::Parser;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Exp <'a> {
    i : &'a !, 
}

impl <'a> Parser<'a> for Exp<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<super::Ast<'a>>, tokens: &'a [cfront_definition::token::Token<'a>]) -> Result<(Self, &'a [cfront_definition::token::Token<'a>]), <Self as Parser<'a>>::E> {
        todo!()
    }
}