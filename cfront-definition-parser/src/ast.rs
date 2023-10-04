pub mod translation_unit;
pub mod external_decl;
pub mod function_definition;
pub mod decl;
pub mod decl_specs;
pub mod r#struct;
pub mod r#enum;
pub mod abstract_declarator;
pub mod pointer;
pub mod struct_decl;
pub mod type_qualifier_list;
pub mod param_list;
pub mod param_decl;
pub mod id_list;
pub mod param_type_list;
pub mod declarator;
pub mod const_exp;
pub mod init_declarator;
pub mod initializer;
pub mod stat;
pub mod exp;
pub mod type_name;
pub mod argument_exp_list;

use std::marker::PhantomData;

use cfront_definition::token::Token;

use self::{translation_unit::TranslationUnit, external_decl::ExternalDecl, r#struct::StructOrUnionSpec, r#enum::{EnumSpec, Enumerator, EnumeratorList}, decl_specs::{StorageClassSpec, TypeSpec, TypeQualifier, DeclSpecs, DeclSpec}, struct_decl::{StructDeclList}, pointer::Pointer, type_qualifier_list::TypeQualifierList, decl::{Decl, DeclList}, param_list::ParamList, declarator::{Declarator, DirectDeclarator}, id_list::IdList, param_decl::ParamDecl, param_type_list::ParamTypeList, const_exp::ConstExp, abstract_declarator::{AbstractDeclarator, DirectAbstractDeclarator}, init_declarator::InitDeclarator, initializer::{Initializer, InitializerList}, stat::{Stat, StatList, CompoundStat, SelectionStat, IterationStat, LabeledStat, JumpStat, ExpStat}, exp::{Exp, BiExp, CastExp, UnaryExp, PostfixExp, PrimaryExp, AssignmentExp, ConditionalExp}, type_name::TypeName};

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
    Enumerator(Enumerator<'a>),
    EnumeratorList(EnumeratorList<'a>),
    StorageClassSpec(StorageClassSpec<'a>),
    TypeSpec(TypeSpec<'a>),
    TypeQualifier(TypeQualifier<'a>),
    DeclSpec(DeclSpec<'a>),
    DeclSpecs(DeclSpecs<'a>),
    StructDeclList(StructDeclList<'a>),
    Pointer(Pointer<'a>),
    TypeQualifierList(TypeQualifierList<'a>),
    Decl(Decl<'a>),
    DeclList(DeclList<'a>),
    ParamList(ParamList<'a>),
    ParamDecl(ParamDecl<'a>),
    ParamTypeList(ParamTypeList<'a>), 
    Declarator(Declarator<'a>),
    DirectDeclarator(DirectDeclarator<'a>), 
    IdList(IdList<'a>),
    AbstractDeclarator(AbstractDeclarator<'a>), 
    DirectAbstractDeclarator(DirectAbstractDeclarator<'a>),
    InitDeclarator(InitDeclarator<'a>), 
    Initializer(Initializer<'a>),
    InitializerList(InitializerList<'a>),
    AssignmentExp(AssignmentExp<'a>), 
    ConditionalExp(ConditionalExp<'a>),
    Exp(Exp<'a>),
    BiExp(BiExp<'a>),
    CastExp(CastExp<'a>),
    UnaryExp(UnaryExp<'a>),
    PostfixExp(PostfixExp<'a>),
    ConstExp(ConstExp<'a>),
    PrimaryExp(PrimaryExp<'a>),
    Stat(Stat<'a>),
    CompoundStat(CompoundStat<'a>),
    SelectionStat(SelectionStat<'a>),
    IterationStat(IterationStat<'a>),
    LabeledStat(LabeledStat<'a>),
    JumpStat(JumpStat<'a>),
    ExpStat(ExpStat<'a>),
    StatList(StatList<'a>),
    TypeName(TypeName<'a>),
}

