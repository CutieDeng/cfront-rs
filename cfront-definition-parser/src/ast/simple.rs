use cfront_definition::token::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct UnaryOperator <'a> (pub Token<'a>); 

#[derive(Debug, PartialEq, Eq)]
pub struct TypeQualifer <'a> (pub Token<'a>); 

