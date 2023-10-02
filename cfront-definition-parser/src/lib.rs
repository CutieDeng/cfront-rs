#![feature(never_type)]

use ast::Ast;
use cfront_definition::token::Token;

pub fn parser<'a> (input: &'a [Token<'a>]) -> Result<Ast<'a>, ()> {
    let _ = input; 
    todo!()
} 

pub mod ast; 

pub trait Parser<'parser> : Sized + 'parser {

    type E; 

    fn parse (stack: &mut Vec<Ast<'parser>>, tokens: &'parser [Token<'parser>]) -> Result<(Self, &'parser [Token<'parser>]), <Self as Parser<'parser>>::E>; 

}