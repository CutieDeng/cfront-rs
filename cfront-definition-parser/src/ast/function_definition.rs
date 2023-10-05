use cfront_definition::token::Token;

use crate::{Parser, ast::{decl_specs::DeclSpecs, declarator::Declarator, decl::DeclList, stat::CompoundStat, AstType}};

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct FunctionDefinition <'a> {
    pub decl_specs: Option<Box<Ast<'a>>>, 
    pub declarator: Box<Ast<'a>>, 
    pub decl_list: Option<Box<Ast<'a>>>, 
    pub compound_statement: Box<Ast<'a>>, 
}

impl <'a> Parser<'a> for FunctionDefinition<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let d = DeclSpecs::parse(stack, tokens); 
        let (p1, r1); 
        match d {
            Ok((p, r)) => {
                p1 = Some(p); 
                r1 = r; 
            }
            Err(_) => {
                p1 = None; 
                r1 = tokens; 
            }
        }
        // #[cfg(debug_assertions)]
        // dbg!(&p1); 
        let d = Declarator::parse(stack, r1); 
        let (p2, r2) = d?; 
        // #[cfg(debug_assertions)]
        // dbg!(&p2);
        let d = DeclList::parse(stack, r2); 
        let (p3, r3); 
        match d {
            Ok((p, r)) => {
                p3 = Some(p); 
                r3 = r; 
            }
            Err(_) => {
                p3 = None; 
                r3 = r2; 
            }
        } 
        // #[cfg(debug_assertions)]
        // dbg!(&p3); 
        let d = CompoundStat::parse(stack, r3); 
        let (p4, r4) = d?;
        #[cfg(debug_assertions)]
        dbg!(&p4); 
        let ans = FunctionDefinition {
            decl_specs: p1.map(|p| Box::new(Ast(AstType::DeclSpecs(p), &tokens[..tokens.len() - r1.len()]))), 
            declarator: Box::new(Ast(AstType::Declarator(p2), &r1[..r1.len() - r2.len()])), 
            decl_list: p3.map(|p| Box::new(Ast(AstType::DeclList(p), &r2[..r2.len() - r3.len()]))), 
            compound_statement: Box::new(Ast(AstType::CompoundStat(p4), &r3[..r3.len() - r4.len()])), 
        }; 
        Ok((ans, r4))
    } 
}
