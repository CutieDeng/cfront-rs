use cfront_definition::token::Token;

use self::r#type::{storage::StorageClassSpec, qualifier::TypeQualifer};

pub mod compound;
pub mod simple;
pub mod r#enum;
pub mod typespecifider;
pub mod pointer;
pub mod list;
pub mod expression;
pub mod declarator;
pub mod r#const;
pub mod r#type;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AstNode <'a> 
    (pub AstType<'a>, pub &'a [Token<'a>]); 

#[derive(Debug, PartialEq, Eq, Clone)] 
pub enum AstType <'a> {
    NoImpl(&'a !), 
    StorageClassSpec(StorageClassSpec), 
    TypeQualifier(TypeQualifer),

}
