use cfront_definition::token::{Token, TokenType};

use crate::{Parser, ast::AstType};

use super::{Ast, id_list::IdList};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Declarator<'a> {
    pub pointer: Option<Box<Ast<'a>>>, 
    pub direct_declarator: Box<Ast<'a>>, 
}

impl <'a> Parser<'a> for Declarator<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [cfront_definition::token::Token<'a>]) -> Result<(Self, &'a [cfront_definition::token::Token<'a>]), <Self as Parser<'a>>::E> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DirectDeclarator<'a> {
    Id(Token<'a>), 
    Parenthesis(Box<Ast<'a>>), 
    EmptyBracket(Box<Ast<'a>>), 
    Bracket { 
        direct_declarator: Box<Ast<'a>>, 
        constant_expr: Box<Ast<'a>>, 
    }, 
    ParamTypeList {
        direct_declarator: Box<Ast<'a>>, 
        param_type_list: Box<Ast<'a>>, 
    }, 
    IdList {
        direct_declarator: Box<Ast<'a>>, 
        id_list: Box<Ast<'a>>, 
    }, 
    EmptyParenthesis(Box<Ast<'a>>), 
}

impl <'a> Parser<'a> for DirectDeclarator<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let first = tokens.first().ok_or(())?;
        let mut this; 
        let mut rst = &tokens[1..];
        match first.token_type {
            TokenType::Parenthesis { is_left: true } => {
                let parse = Declarator::parse(stack, rst)?; 
                let r2 = parse.1; 
                let f2 = r2.first().ok_or(())?;
                let Token { token_type: TokenType::Parenthesis { is_left: false }, .. } = f2 else { return Err(()); };  
                let len = rst.len() - parse.1.len(); 
                let t = Ast(AstType::Declarator(parse.0), &tokens[..len]); 
                rst = &r2[1..];
                this = DirectDeclarator::Parenthesis(Box::new(t)); 
            }
            TokenType::Identifier(_) => {
                this = DirectDeclarator::Id(first.clone()); 
            }
            _ => return Err(()),  
        }
        loop {
            let Some(first) = rst.first() else { break }; 
            let first_type = &first.token_type; 
            match first_type {
                TokenType::Parenthesis { is_left: true } => {
                    match rst.get(1) {
                        Some(f2) => {
                            let f2type = &f2.token_type; 
                            match f2type {
                                TokenType::Parenthesis { is_left: false } => {
                                    rst = &rst[2..]; 
                                    let len = tokens.len() - rst.len(); 
                                    let t = Ast(AstType::DirectDeclarator(this), &tokens[..len]); 
                                    this = DirectDeclarator::EmptyParenthesis(Box::new(t)); 
                                }
                                _ => {
                                    unimplemented!()
                                }
                            }
                        }
                        None => break, 
                    }
                }
                TokenType::Bracket { is_left: true } => {
                    match rst.get(1) {
                        Some(f2) => {
                            let f2type = &f2.token_type;         
                            match f2type {
                                TokenType::Bracket { is_left: false } => {
                                    rst = &rst[2..]; 
                                    let len = tokens.len() - rst.len(); 
                                    let t = Ast(AstType::DirectDeclarator(this), &tokens[..len]); 
                                    this = DirectDeclarator::EmptyBracket(Box::new(t)); 
                                }
                                TokenType::Identifier(_) => {
                                    let tmp = &rst[1..]; 
                                    let Ok(parse) = IdList::parse(stack, tmp) else { break }; 
                                    let r2 = parse.1; 
                                    let Some(f2) = r2.first() else { break }; 
                                    let f2type = &f2.token_type;
                                    match f2type {
                                        TokenType::Bracket { is_left: false } => {
                                            let len = tokens.len() - rst.len(); 
                                            let t = Ast(AstType::DirectDeclarator(this), &tokens[..len]); 
                                            let len = tmp.len() - r2.len(); 
                                            let t2 = Ast(AstType::IdList(parse.0), &tmp[..len]);  
                                            this = DirectDeclarator::IdList { direct_declarator: Box::new(t), id_list: Box::new(t2) }; 
                                            rst = &r2[1..]; 
                                        }
                                        _ => break, 
                                    }
                                }
                                _ => {
                                    unimplemented!()
                                }
                            }
                        }
                        None => break, 
                    }
                }
                _ => break, 
            }
        }
        return Ok((this, rst)); 
    }
}