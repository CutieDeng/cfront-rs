#![feature(never_type)]

use ast::Ast;
use cfront_definition::token::Token;

use crate::ast::{translation_unit::TranslationUnit, AstType};

pub fn parser<'a> (input: &'a [Token<'a>]) -> Result<Ast<'a>, ()> {
    let p = TranslationUnit::parse(&mut Vec::new(), input)?; 
    let len = input.len() - p.1.len(); 
    let a = Ast(AstType::TranslationUnit(p.0), &input[..len]); 
    return Ok(a); 
} 

pub mod ast; 

pub trait Parser<'parser> : Sized + 'parser {

    type E; 

    fn parse (stack: &mut Vec<Ast<'parser>>, tokens: &'parser [Token<'parser>]) -> Result<(Self, &'parser [Token<'parser>]), <Self as Parser<'parser>>::E>; 

}