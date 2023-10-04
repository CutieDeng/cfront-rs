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
pub mod struct_decl;
pub mod type_qualifier_list;
pub mod param_list;
pub mod param_decl;
pub mod id_list;
pub mod param_type_list;
pub mod declarator;

use std::marker::PhantomData;

use cfront_definition::token::Token;

use self::{translation_unit::TranslationUnit, external_decl::ExternalDecl, r#struct::StructOrUnionSpec, r#enum::EnumSpec, decl_specs::{StorageClassSpec, TypeSpec, TypeQualifier, DeclSpecs, DeclSpec}, struct_decl::{StructDeclList}, pointer::Pointer, type_qualifier_list::TypeQualifierList, decl::Decl, param_list::ParamList, declarator::{Declarator, DirectDeclarator}, id_list::IdList, param_decl::ParamDecl};

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
    StructDeclList(StructDeclList<'a>),
    Pointer(Pointer<'a>),
    TypeQualifierList(TypeQualifierList<'a>),
    Decl(Decl<'a>),
    ParamList(ParamList<'a>),
    ParamDecl(ParamDecl<'a>),
    Declarator(Declarator<'a>),
    DirectDeclarator(DirectDeclarator<'a>), 
    IdList(IdList<'a>),
}
