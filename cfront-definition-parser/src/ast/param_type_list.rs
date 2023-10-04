use cfront_definition::token::{Token, TokenType};

use crate::{Parser, ast::{param_list::ParamList, AstType}};

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParamTypeList<'a> {
    pub param_list: Box<Ast<'a>>,
    pub ellipsis: bool, 
}

impl <'a> Parser<'a> for ParamTypeList<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [cfront_definition::token::Token<'a>]) -> Result<(Self, &'a [cfront_definition::token::Token<'a>]), <Self as Parser<'a>>::E> {
        let parse = ParamList::parse(stack, tokens)?; 
        let mut rst = parse.1; 
        let used = tokens.len() - rst.len(); 
        let mut param = parse.0; 
        let par; 
        let ellipsis; 
        if let Some(Token { token_type: TokenType::Operator("..."), .. }) = rst.first() {
            param.trailing_comma = false; 
            ellipsis = true; 
            rst = &rst[1..]; 
            par = Ast(AstType::ParamList(param), &tokens[..used - 1]); 
        } else {
            ellipsis = false; 
            par = Ast(AstType::ParamList(param), &tokens[..used]);  
        }
        let par = Box::new(par); 
        let param_type_list = ParamTypeList {
            param_list: par, 
            ellipsis,
        }; 
        return Ok((param_type_list, rst)); 
    }
}