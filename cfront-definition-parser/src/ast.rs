use std::marker::PhantomData;

use cfront_definition::token::Token;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AstNode <'a> 
    (pub AstType<'a>, pub &'a [Token<'a>]); 

#[derive(Debug, PartialEq, Eq, Clone)] 
pub enum AstType <'a> {
    NoImpl(PhantomData<&'a !>), 
}
