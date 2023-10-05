use cfront_definition::token::{Token, TokenType};

use crate::{Parser, ast::{AstType, const_exp::ConstExp}};

use super::{Ast, id_list::IdList, param_type_list::ParamTypeList, pointer::Pointer};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Declarator<'a> {
    pub pointer: Option<Box<Ast<'a>>>, 
    pub direct_declarator: Box<Ast<'a>>, 
}

impl <'a> Parser<'a> for Declarator<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let first = tokens.first().ok_or(())?;
        let mut rst = tokens;
        let pointer; 
        if first.token_type == TokenType::Operator("*") {
            let parse = Pointer::parse(stack, rst).unwrap(); 
            let r2 = parse.1; 
            rst = r2; 
            let len = tokens.len() - r2.len(); 
            let t = Ast(AstType::Pointer(parse.0), &tokens[..len]); 
            pointer = Some(Box::new(t)); 
        } else {
            pointer = None; 
        }
        let dd = DirectDeclarator::parse(stack, rst)?;
        // #[cfg(debug_assertions)]
        // dbg!(&dd); 
        let len = rst.len() - dd.1.len(); 
        let t = Ast(AstType::DirectDeclarator(dd.0), &rst[..len]);
        let declarator = Declarator {
            pointer, 
            direct_declarator: Box::new(t), 
        }; 
        rst = dd.1; 
        return Ok((declarator, rst)); 
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
        'outer: 
        loop {
            let Some(first) = rst.first() else { break }; 
            let first_type = &first.token_type; 
            match first_type {
                TokenType::Bracket { is_left: true } => {
                    match rst.get(1) {
                        Some(f2) => {
                            let f2type = &f2.token_type; 
                            match f2type {
                                TokenType::Brace { is_left: false } => {
                                    rst = &rst[2..]; 
                                    let len = tokens.len() - rst.len(); 
                                    let t = Ast(AstType::DirectDeclarator(this), &tokens[..len]); 
                                    this = DirectDeclarator::EmptyBracket(Box::new(t)); 
                                }
                                _ => {
                                    let p = ConstExp::parse(stack, rst);
                                    let Ok((p, r)) = p else { break }; 
                                    let Some(f) = r.first() else { break }; 
                                    let ftype = &f.token_type; 
                                    let TokenType::Brace { is_left: false } = ftype else { break }; 
                                    let len = tokens.len() - rst.len(); 
                                    let t = Ast(AstType::DirectDeclarator(this), &tokens[..len]); 
                                    let len = rst.len() - r.len(); 
                                    let t2 = Ast(AstType::ConstExp(p), &rst[..len]); 
                                    this = DirectDeclarator::Bracket { direct_declarator: Box::new(t), constant_expr: Box::new(t2) }; 
                                    rst = &r[1..];
                                }
                            }
                        }
                        None => break, 
                    }
                }
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
                                    let tmp = &rst[1..]; 
                                    'inner_scope: {
                                        let Ok(parse) = IdList::parse(stack, tmp) else { break 'inner_scope }; 
                                        let r2 = parse.1; 
                                        let Some(f2) = r2.first() else { break 'inner_scope }; 
                                        let f2type = &f2.token_type;
                                        match f2type {
                                            TokenType::Parenthesis { is_left: false } => {
                                                let len = tokens.len() - rst.len(); 
                                                let t = Ast(AstType::DirectDeclarator(this), &tokens[..len]); 
                                                let len = tmp.len() - r2.len(); 
                                                let t2 = Ast(AstType::IdList(parse.0), &tmp[..len]);  
                                                this = DirectDeclarator::IdList { direct_declarator: Box::new(t), id_list: Box::new(t2) }; 
                                                rst = &r2[1..]; 
                                                continue 'outer;
                                            }
                                            _ => break 'inner_scope, 
                                        }
                                    }
                                    let Ok(parse) = ParamTypeList::parse(stack, tmp) else { break };  
                                    let r2 = parse.1; 
                                    let Some(f2) = r2.first() else { break }; 
                                    let f2type = &f2.token_type; 
                                    if let TokenType::Parenthesis { is_left: false } = f2type {
                                        let len = tokens.len() - rst.len(); 
                                        let t = Ast(AstType::DirectDeclarator(this), &tokens[..len]); 
                                        let len = tmp.len() - r2.len(); 
                                        let t2 = Ast(AstType::ParamTypeList(parse.0), &tmp[..len]); 
                                        this = DirectDeclarator::ParamTypeList { direct_declarator: Box::new(t), param_type_list: Box::new(t2) }; 
                                        rst = &r2[1..]; 
                                        continue 'outer; 
                                    } else {
                                        break; 
                                    }
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