use cfront_definition::token::{Token, TokenType};

use crate::{Parser, ast::initializer::Initializer};

use super::{Ast, AstType, declarator::Declarator};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InitDeclarator<'a> {
    pub declarator: Box<Ast<'a>>, 
    pub initializer: Option<Box<Ast<'a>>>, 
}

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct InitDeclaratorList<'a> { 
    pub init_declarators: Vec<Ast<'a>>,
    pub trailling_comma: bool, 
}

impl <'a> Parser<'a> for InitDeclaratorList<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let mut ans = Vec::new(); 
        let mut rst = tokens; 
        let mut trailling_comma = false; 
        loop {
            let Ok((p, r)) = InitDeclarator::parse(stack, rst) else { break }; 
            let p = Ast(AstType::InitDeclarator(p), &rst[..rst.len() - r.len()]); 
            ans.push(p); 
            rst = r; 
            trailling_comma = false;
            let Some(comma) = rst.first() else { break }; 
            let comma = &comma.token_type; 
            let TokenType::Operator(",") = comma else { break }; 
            rst = &rst[1..];  
            trailling_comma = true; 
        }
        let list = InitDeclaratorList {
            init_declarators: ans, 
            trailling_comma, 
        }; 
        return Ok((list, rst)); 
    }
}

impl <'a> Parser<'a> for InitDeclarator<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        #[cfg(debug_assertions)]
        eprintln!("InitDeclarator");
        let (declarator, r) = Declarator::parse(stack, tokens)?; 
        #[cfg(debug_assertions)]
        eprintln!("InitDeclarator-declarator: {declarator:?}");
        let declarator = Box::new(Ast(AstType::Declarator(declarator), &tokens[..tokens.len() - r.len()])); 
        let mut initializer = None; 
        let mut rst = r; 
        'eq: {
            let Some(eq) = r.first() else { break 'eq }; 
            let eq = &eq.token_type; 
            let TokenType::Operator("=") = eq else { break 'eq }; 
            let r2 = &r[1..];
            let Ok((init, r3)) = Initializer::parse(stack, r2) else { break 'eq }; 
            initializer = Some(Box::new(Ast(AstType::Initializer(init), &r2[..r2.len() - r3.len()])));
            rst = r3; 
        }
        let ans = InitDeclarator {
            declarator, 
            initializer, 
        }; 
        return Ok((ans, rst)); 
    }
}