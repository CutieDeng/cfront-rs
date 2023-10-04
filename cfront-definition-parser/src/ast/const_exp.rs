use cfront_definition::token::Token;

use crate::Parser;

use super::{exp::ConditionalExp, Ast};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConstExp <'a> ( pub ConditionalExp<'a> ); 

impl <'a> Parser<'a> for ConstExp <'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        ConditionalExp::parse(stack, tokens).map(|(conditional_exp, tokens)| (Self(conditional_exp), tokens))
    }
}