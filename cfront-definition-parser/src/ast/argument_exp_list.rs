use cfront_definition::token::{Token, TokenType};

use crate::ast::{exp::AssignmentExp, AstType};
use crate::Parser;

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArgumentExpList<'a> (pub Vec<Ast<'a>>);

impl <'a> Parser<'a> for ArgumentExpList<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let mut ans = Vec::new(); 
        let mut rst = tokens; 
        let mut start = false; 
        loop {
            if start {
                let f = rst.first(); 
                if let Some(Token { token_type: TokenType::Operator(","), .. }) = f { 
                    rst = &rst[1..]; 
                } else {
                    break; 
                } 
            }
            match AssignmentExp::parse(stack, rst) {
                Ok((exp, rest)) => {
                    let a = Ast(AstType::AssignmentExp(exp), &rst[..rst.len() - rest.len()]);
                    ans.push(a); 
                    rst = rest; 
                }, 
                Err(_) => return Err(()), 
            }
            start = true;     
        } 
        Ok((Self(ans), rst)) 
    }
}
