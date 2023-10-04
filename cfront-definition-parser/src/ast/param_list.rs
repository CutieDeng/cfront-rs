use cfront_definition::token::{Token, TokenType};

use crate::{Parser, ast::{param_decl::ParamDecl, AstType}};

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct ParamList <'a> { 
    pub params: Vec<Ast<'a>>, 
    pub trailing_comma: bool, 
} 

impl <'a> Parser<'a> for ParamList<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let mut ans = Vec::new(); 
        let mut rst = tokens; 
        let mut trailing_comma = false; 
        loop {
            let Ok(parse) = ParamDecl::parse(stack, rst) else { break }; 
            let pd = Ast(AstType::ParamDecl(parse.0), &rst[..rst.len() - parse.1.len()]); 
            ans.push(pd); 
            trailing_comma = false; 
            rst = parse.1; 
            let Some(first) = rst.first() else { break }; 
            let ft = &first.token_type;
            if let TokenType::Operator(",") = ft {
                rst = &rst[1..]; 
                trailing_comma = true; 
            } else {
                break ; 
            }
        }
        let param_list = ParamList {
            params: ans, 
            trailing_comma, 
        };  
        return Ok((param_list, rst)); 
    }
}