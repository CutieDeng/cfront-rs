pub mod translation_unit;
pub mod external_decl;
pub mod function_definition;
pub mod decl;
pub mod decl_specs;
pub mod storage_class_spec;

use std::marker::PhantomData;

use cfront_definition::token::Token;

use self::{translation_unit::TranslationUnit, external_decl::ExternalDecl};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast<'a> 
    (pub AstType<'a>, pub &'a [Token<'a>]); 

#[derive(Debug, PartialEq, Eq, Clone)] 
pub enum AstType <'a> {
    NoImpl(PhantomData<&'a !>), 
    TranslationUnit(TranslationUnit<'a>),
    ExternalDecl(ExternalDecl<'a>),
}
