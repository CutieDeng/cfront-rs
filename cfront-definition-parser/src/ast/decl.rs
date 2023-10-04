use cfront_definition::token::Token;

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
        todo!()
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