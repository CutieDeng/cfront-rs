use cfront_definition::token::Token;

use crate::{Parser, ast::{decl_specs::{TypeQualifier, TypeSpec}, AstType}};

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct StructDeclList <'a> (pub Vec<Ast<'a>>); 

impl <'a> Parser<'a> for StructDeclList<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct SpecQualifierList<'a> (pub Vec<Ast<'a>>); 

impl <'a> Parser<'a> for SpecQualifierList<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let mut rst = tokens; 
        let mut ans = Vec::new(); 
        loop {
            let tq = TypeQualifier::parse(stack, rst); 
            match tq {
                Ok((tq, r)) => {
                    let len = rst.len() - r.len(); 
                    ans.push(Ast(AstType::TypeQualifier(tq), &rst[..len])); 
                    rst = r; 
                    continue ; 
                }, 
                Err(_) => (), 
            } 
            let p = TypeSpec::parse(stack, rst); 
            match p {
                Ok((p, r)) => {
                    let len = rst.len() - r.len(); 
                    ans.push(Ast(AstType::TypeSpec(p), &rst[..len])); 
                    rst = r; 
                } 
                Err(_) => break, 
            }
        }
        if ans.is_empty() {
            return Err(());  
        } else {
            return Ok((SpecQualifierList(ans), rst));  
        }
    }
}

// struct_declarator_list
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StructDeclaratorList<'a> ( &'a ! ); 

impl <'a> Parser<'a> for StructDeclaratorList<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        todo!()
    }
} 

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct StructDeclarator <'a> {
    pub declarator: Option<Box<Ast<'a>>>,
    pub const_expr: Option<Box<Ast<'a>>>, 
}