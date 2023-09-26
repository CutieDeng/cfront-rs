#![feature(never_type)]

use cfront_definition::token::Token;

pub fn parser(input: &[Token<'_>]) -> Result<(), ()> {
    let _ = input; 
    Ok(())
} 

pub mod ast; 

pub trait Parser<'parser> : Sized + 'parser {

    type E; 

    fn parse (tokens: &'parser [Token<'parser>]) -> Result<(Self, &'parser [Token<'parser>]), <Self as Parser>::E>; 

}