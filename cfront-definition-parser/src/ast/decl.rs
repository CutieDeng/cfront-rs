use cfront_definition::token::{Token, TokenType};

use crate::ast::init_declarator::InitDeclaratorList;
use crate::ast::decl_specs::DeclSpecs;
use crate::Parser;

use super::{Ast, AstType};

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct Decl <'a> {
    pub decl_specs: Box<Ast<'a>>,
    pub init_declarator_list: Option<Box<Ast<'a>>>,
}

impl <'a> Parser<'a> for Decl<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let (p, r) = DeclSpecs::parse(stack, tokens)?;
        let p2 = InitDeclaratorList::parse(stack, r);
        let p3; 
        let r2; 
        match p2 {
            Ok((p2, r)) => {
                p3 = Some(p2); 
                r2 = r; 
            }
            Err(_) => {
                p3 = None; 
                r2 = r; 
            }
        }
        let f = r2.first().ok_or(())?; 
        let Token { token_type: TokenType::Operator(";"), .. } = f else { return Err(()) };  
        let decl_specs = Box::new(Ast(AstType::DeclSpecs(p), &tokens[..tokens.len() - r.len()])); 
        let init_declarator_list = p3.map(|p| Box::new(Ast(AstType::InitDeclaratorList(p), &r[..r.len() - r2.len()])));  
        let ans = Decl {
            decl_specs, 
            init_declarator_list, 
        }; 
        return Ok((ans, &r2[1..])); 
    } 
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DeclList<'a> (pub Vec<Ast<'a>>); 

impl <'a> Parser<'a> for DeclList<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let mut rst = tokens; 
        let mut ans = Vec::new(); 
        loop {
            let parse = Decl::parse(stack, rst); 
            let Ok(parse) = parse else { break };
            let (parse, rst2) = parse; 
            rst = rst2; 
            ans.push(Ast(AstType::Decl(parse), &tokens[..tokens.len() - rst.len()])); 
        } 
        if ans.is_empty() {
            return Err(());
        } else {
            return Ok((DeclList(ans), rst)); 
        }
    }
} 