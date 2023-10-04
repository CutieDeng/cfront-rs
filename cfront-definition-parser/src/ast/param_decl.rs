use crate::{Parser, ast::{decl_specs::DeclSpecs, declarator::Declarator, abstract_declarator::AbstractDeclarator}};

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParamDecl <'a> {
    pub decl_specs: Box<Ast<'a>>, 
    pub declarator: Option<Box<Ast<'a>>>, 
    pub abstract_declarator: Option<Box<Ast<'a>>>,     
}

impl <'a> Parser<'a> for ParamDecl<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<super::Ast<'a>>, tokens: &'a [cfront_definition::token::Token<'a>]) -> Result<(Self, &'a [cfront_definition::token::Token<'a>]), <Self as Parser<'a>>::E> {
        let (declspecs, r) = DeclSpecs::parse(stack, tokens)?; 
        let decl_specs = Box::new(Ast(super::AstType::DeclSpecs(declspecs), &tokens[..tokens.len() - r.len()])); 
        let d = Declarator::parse(stack, r); 
        let a = AbstractDeclarator::parse(stack, r); 
        let select_declarator; 
        match (&d, &a) {
            (Ok((_, l)), Ok((_, r))) => {
                if l.len() > r.len() {
                    select_declarator = false; 
                } else {
                    select_declarator = true; 
                } 
            }
            (Ok(_), Err(_)) => 
                select_declarator = true,
            (Err(_), Ok(_)) => 
                select_declarator = false, 
            (Err(_), Err(_)) => {
                return Ok((ParamDecl {
                    decl_specs, 
                    declarator: None, 
                    abstract_declarator: None, 
                }, r)); 
            }
        } 
        if select_declarator {
            let (declarator, r2) = d.unwrap(); 
            let declarator = Box::new(Ast(super::AstType::Declarator(declarator), &r[..r.len() - r2.len()]));
            return Ok((ParamDecl {
                decl_specs, 
                declarator: Some(declarator), 
                abstract_declarator: None, 
            }, r)); 
        } else {
            let (abstract_declarator, r) = a.unwrap(); 
            let abstract_declarator = Box::new(Ast(super::AstType::AbstractDeclarator(abstract_declarator), &r[..r.len() - r.len()])); 
            return Ok((ParamDecl {
                decl_specs, 
                declarator: None, 
                abstract_declarator: Some(abstract_declarator), 
            }, r));    
        } 
    }
}