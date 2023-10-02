pub mod translation_unit;
pub mod external_decl;
pub mod function_definition;
pub mod decl;
pub mod decl_specs;
pub mod r#struct;
pub mod r#enum;
pub mod typename;
pub mod abstract_declarator;
pub mod direct_abstract_declarator;
pub mod pointer;

use std::marker::PhantomData;

use cfront_definition::token::Token;

use self::{translation_unit::TranslationUnit, external_decl::ExternalDecl, r#struct::StructOrUnionSpec, r#enum::EnumSpec, decl_specs::{StorageClassSpec, TypeSpec, TypeQualifier, DeclSpecs, DeclSpec}};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast<'a> 
    (pub AstType<'a>, pub &'a [Token<'a>]); 

#[derive(Debug, PartialEq, Eq, Clone)] 
pub enum AstType <'a> {
    NoImpl(PhantomData<&'a !>), 
    TranslationUnit(TranslationUnit<'a>),
    ExternalDecl(ExternalDecl<'a>),
    StructOrUnionSpec(StructOrUnionSpec<'a>), 
    EnumSpec(EnumSpec<'a>),
    StorageClassSpec(StorageClassSpec<'a>),
    TypeSpec(TypeSpec<'a>),
    TypeQualifier(TypeQualifier<'a>),
    DeclSpec(DeclSpec<'a>),
}
