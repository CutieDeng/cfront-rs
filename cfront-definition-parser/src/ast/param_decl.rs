use crate::Parser;

pub struct ParamDecl <'a> {
    a: &'a !, 
}

impl <'a> Parser<'a> for ParamDecl<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<super::Ast<'a>>, tokens: &'a [cfront_definition::token::Token<'a>]) -> Result<(Self, &'a [cfront_definition::token::Token<'a>]), <Self as Parser<'a>>::E> {
        todo!()
    }
}