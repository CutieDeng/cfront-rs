pub mod enumerator;
pub mod qualifier;
pub mod r#struct;
pub mod spec;
pub mod storage;
pub mod r#enum;

use cfront_definition::token::{Token, TokenType};

use super::AstNode;

pub enum TypeDefName <'a> {
    Identifier(TokenType<'a>),
}

pub enum TypeSpec <'a> {
    RawType ( Token<'a> ),
    StructSpec (StructSpec<'a>),

}

pub struct StructSpec <'a> {
    pub r#struct: AstNode<'a>, 
    pub identifier: Option<Token<'a>>, 
    pub struct_decl_list: Option<StructDeclList<'a>>, 
}

pub struct StructDeclList <'a> ( 
    /// Only contains [`StructDecl`] instances. 
    pub Vec<AstNode<'a>>
);

pub struct StructDecl {

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Struct {
    Struct, 
    Union,
}

pub struct EnumSpec <'a> {
    pub identifier: Option<Token<'a>>, 
    /// Only contains [`EnumeratorList`]
    pub list: Option<AstNode<'a>>
}

pub struct EnumeratorList <'a> {
    /// Only contains `Enumerator` instances. 
    pub list: Vec<AstNode<'a>>,
    pub trailling_comma: Option<Token<'a>>, 
}

pub struct Enumerator <'a> (pub Token<'a>); 

pub enum TypeSpec <'a> {
    RawType ( Token<'a> ),
    
}