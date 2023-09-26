use cfront_definition::token::Token;

use crate::ast::AstNode;

pub struct EnumSpec <'a> {
    pub identity: Option<Box<AstNode<'a>>>, 
    pub list: Option<Box<AstNode<'a>>>,
}

pub struct EnumeratorList <'a> (pub Vec<AstNode<'a>>); 

pub struct Enumerator <'a> {
    pub identity: Token<'a>, 
    pub value: Option<Box<AstNode<'a>>>, 
}