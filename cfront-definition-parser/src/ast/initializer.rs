use cfront_definition::token::{Token, TokenType};

use crate::Parser;

use super::{Ast, AstType, exp::AssignmentExp};

#[derive(Debug, PartialEq, Eq, Clone)] 
pub enum Initializer <'a> {
    AssignmentExpr(Box<Ast<'a>>), 
    BracedInitList(Box<Ast<'a>>),  
}

impl <'a> Parser<'a> for Initializer<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let first = tokens.first().ok_or(())?; 
        let ft = &first.token_type;
        let rst; 
        let ans; 
        if let TokenType::Brace { is_left: true } = ft {
            let r = &tokens[1..];
            let (list, r2) = InitializerList::parse(stack, r)?; 
            let f = r2.first().ok_or(())?;
            let ft = &f.token_type; 
            let TokenType::Brace { is_left: false } = ft else { return Err(()) };
            let ast = Ast(AstType::InitializerList(list), &r[..r.len() - r2.len()]);
            ans = Initializer::BracedInitList(Box::new(ast)); 
            rst = &r2[1..]; 
        } else {
            let (expr, r) = AssignmentExp::parse(stack, tokens)?; 
            let ast = Ast(AstType::AssignmentExp(expr), &tokens[..tokens.len() - r.len()]); 
            ans = Initializer::AssignmentExpr(Box::new(ast)); 
            rst = r;  
        }
        return Ok((ans, rst)); 
    }
} 

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct InitializerList <'a> {
    pub initializers: Vec<Ast<'a>>, 
    pub trailling_comma: bool,  
}

impl <'a> Parser<'a> for InitializerList<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let mut rst = tokens; 
        let mut ans = Vec::new(); 
        let mut trailling_comma = false; 
        loop {
            let parse = Initializer::parse(stack, rst); 
            let Ok(parse) = parse else { break }; 
            let (parse, rst2) = parse; 
            ans.push(Ast(AstType::Initializer(parse), &rst[..rst.len() - rst2.len()]));
            rst = rst2; 
            trailling_comma = false; 
            let Some(comma) = rst.first() else { break }; 
            let comma = &comma.token_type; 
            let TokenType::Operator(",") = comma else { break }; 
            rst = &rst[1..]; 
            trailling_comma = true; 
        } 
        let list = InitializerList {
            initializers: ans, 
            trailling_comma, 
        }; 
        return Ok((list, rst)); 
    }
} 