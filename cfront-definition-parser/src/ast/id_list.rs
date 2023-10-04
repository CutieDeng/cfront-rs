use cfront_definition::token::{Token, TokenType};

use crate::Parser; 

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IdList<'a> {
    pub id: Vec<Token<'a>>, 
    pub trailing_comma: bool, 
}

impl <'a> Parser<'a> for IdList<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let mut ans = Vec::new(); 
        let mut rst = tokens; 
        let mut trailling = false;  
        loop {
            let Some(first) = rst.first() else { break };
            let TokenType::Identifier(_) = first.token_type else { break }; 
            ans.push(first.clone());  
            rst = &rst[1..]; 
            trailling = false; 
            let Some(first) = rst.first() else { break }; 
            let TokenType::Operator(",") = first.token_type else { break }; 
            rst = &rst[1..]; 
            trailling = true; 
        }
        let id_list = IdList {
            id: ans, 
            trailing_comma: trailling, 
        }; 
        return Ok((id_list, rst)); 
    }
}