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

use std::fmt::Debug;

use cfront_definition::token::Token;

use self::{translation_unit::TranslationUnit, external_decl::ExternalDecl, r#struct::StructOrUnionSpec, r#enum::{EnumSpec, Enumerator, EnumeratorList}, decl_specs::{StorageClassSpec, TypeSpec, TypeQualifier, DeclSpecs, DeclSpec}, struct_decl::{StructDeclList, SpecQualifierList, StructDeclarator, StructDeclaratorList, StructDecl}, pointer::Pointer, type_qualifier_list::TypeQualifierList, decl::{Decl, DeclList}, param_list::ParamList, declarator::{Declarator, DirectDeclarator}, id_list::IdList, param_decl::ParamDecl, param_type_list::ParamTypeList, const_exp::ConstExp, abstract_declarator::{AbstractDeclarator, DirectAbstractDeclarator}, init_declarator::{InitDeclarator, InitDeclaratorList}, initializer::{Initializer, InitializerList}, stat::{Stat, StatList, CompoundStat, SelectionStat, IterationStat, LabeledStat, JumpStat, ExpStat}, exp::{Exp, BiExp, CastExp, UnaryExp, PostfixExp, PrimaryExp, AssignmentExp, ConditionalExp}, type_name::TypeName, function_definition::FunctionDefinition};

#[derive(PartialEq, Eq, Clone)]
pub struct Ast<'a> 
    (pub AstType<'a>, pub &'a [Token<'a>]); 

impl <'a> Debug for Ast<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(PartialEq, Eq, Clone)] 
pub enum AstType <'a> {
    TranslationUnit(TranslationUnit<'a>),
    ExternalDecl(ExternalDecl<'a>),
    FunctionDefinition(FunctionDefinition<'a>),
    StructOrUnionSpec(StructOrUnionSpec<'a>), 
    EnumSpec(EnumSpec<'a>),
    Enumerator(Enumerator<'a>),
    EnumeratorList(EnumeratorList<'a>),
    StorageClassSpec(StorageClassSpec<'a>),
    TypeSpec(TypeSpec<'a>),
    TypeQualifier(TypeQualifier<'a>),
    DeclSpec(DeclSpec<'a>),
    DeclSpecs(DeclSpecs<'a>),
    StructDecl(StructDecl<'a>),
    StructDeclList(StructDeclList<'a>),
    StructDeclaratorList(StructDeclaratorList<'a>),
    StructDeclarator(StructDeclarator<'a>),
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
    InitDeclaratorList(InitDeclaratorList<'a>), 
    SpecQualifierList(SpecQualifierList<'a>),
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

impl <'a> AstType<'a> {
    pub fn as_debug_inner(&self) -> &dyn Debug {
        match self {
            AstType::TranslationUnit(a) => return a, 
            AstType::ExternalDecl(a) => return a, 
            AstType::FunctionDefinition(a) => return a, 
            AstType::StructOrUnionSpec(a) => return a, 
            AstType::EnumSpec(a) => return a, 
            AstType::Enumerator(a) => return a, 
            AstType::EnumeratorList(a) => return a,
            AstType::StorageClassSpec(a) => return a, 
            AstType::TypeSpec(a) => return a, 
            AstType::TypeQualifier(a) => return a, 
            AstType::DeclSpec(a) => return a, 
            AstType::DeclSpecs(a) => return a, 
            AstType::StructDecl(a) => return a, 
            AstType::StructDeclList(a) => return a, 
            AstType::StructDeclaratorList(a) => return a, 
            AstType::StructDeclarator(a) => return a, 
            AstType::Pointer(a) => return a, 
            AstType::TypeQualifierList(a) => return a, 
            AstType::Decl(a) => return a, 
            AstType::DeclList(a) => return a, 
            AstType::ParamList(a) => return a, 
            AstType::ParamDecl(a) => return a, 
            AstType::ParamTypeList(a) => return a, 
            AstType::Declarator(a) => return a, 
            AstType::DirectDeclarator(a) => return a, 
            AstType::IdList(a) => return a, 
            AstType::AbstractDeclarator(a) => return a, 
            AstType::DirectAbstractDeclarator(a) => return a, 
            AstType::InitDeclarator(a) => return a, 
            AstType::Initializer(a) => return a, 
            AstType::InitializerList(a) => return a, 
            AstType::InitDeclaratorList(a) => return a, 
            AstType::SpecQualifierList(a) => return a, 
            AstType::AssignmentExp(a) => return a, 
            AstType::ConditionalExp(a) => return a, 
            AstType::Exp(a) => return a, 
            AstType::BiExp(a) => return a, 
            AstType::CastExp(a) => return a, 
            AstType::UnaryExp(a) => return a,
            AstType::PostfixExp(a) => return a,
            AstType::ConstExp(a) => return a,
            AstType::PrimaryExp(a) => return a,
            AstType::Stat(a) => return a,
            AstType::CompoundStat(a) => return a,
            AstType::SelectionStat(a) => return a,
            AstType::IterationStat(a) => return a,
            AstType::LabeledStat(a) => return a,
            AstType::JumpStat(a) => return a,
            AstType::ExpStat(a) => return a,
            AstType::StatList(a) => return a,
            AstType::TypeName(a) => return a,
        }
    }
}

impl <'a> Debug for AstType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_debug_inner().fmt(f) 
    }
}