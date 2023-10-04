use cfront_definition::token::{Token, TokenType};

use crate::{Parser, ast::{AstType, param_type_list::ParamTypeList, pointer::Pointer}};

use super::{Ast, const_exp::ConstExp};

#[derive(Debug, PartialEq, Eq, Clone, )] 
pub struct AbstractDeclarator <'a> {
    pub pointer: Option<Box<Ast<'a>>>, 
    pub direct_abstract_declarator: Option<Box<Ast<'a>>>, 
}

impl <'a> Parser<'a> for AbstractDeclarator<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let mut rst = tokens; 
        let pointer = Pointer::parse(stack, tokens).ok(); 
        let pointer = pointer.map(|f| {
            let r = Box::new(Ast(AstType::Pointer(f.0), &rst[..rst.len() - f.1.len()])); 
            rst = f.1; 
            r 
        }); 
        let direct_abstract_declarator = DirectAbstractDeclarator::parse(stack, rst).ok();  
        let direct_abstract_declarator = direct_abstract_declarator.map(|f| {
            let r = Box::new(Ast(AstType::DirectAbstractDeclarator(f.0), &rst[..rst.len() - f.1.len()])); 
            rst = f.1; 
            r 
        }); 
        if pointer.is_none() && direct_abstract_declarator.is_none() {
            return Err(()); 
        } 
        let a = AbstractDeclarator {
            pointer, 
            direct_abstract_declarator, 
        }; 
        return Ok((a, rst)); 
    } 
} 

#[derive(Debug, PartialEq, Eq, Clone, )]
pub enum DirectAbstractDeclarator<'a> {
    EmptyBracket(Option<Box<Ast<'a>>>), 
    EmptyParenthesis(Option<Box<Ast<'a>>>), 
    ConstExp {
        direct_abstract_declarator: Option<Box<Ast<'a>>>, 
        const_exp: Box<Ast<'a>>, 
    }, 
    ParamTypeList {
        direct_abstract_declarator: Option<Box<Ast<'a>>>, 
        param_type_list: Box<Ast<'a>>, 
    }, 
    AbstractDeclarator (Box<Ast<'a>>),
}

impl <'a> Parser<'a> for DirectAbstractDeclarator<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let mut this; 
        let mut rst = tokens; 
        let first = tokens.first().ok_or(())?; 
        let ft = &first.token_type; 
        'out:
        {
            match ft {
                TokenType::Parenthesis { is_left: true } => {
                    let r = &rst[1..]; 
                    'empty: {
                        let Some(f) = r.first() else { break 'empty; }; 
                        let ft = &f.token_type; 
                        let TokenType::Parenthesis { is_left: false } = ft else { break 'empty; }; 
                        this = DirectAbstractDeclarator::EmptyParenthesis(None); 
                        rst = &r[1..];
                        break 'out; 
                    }
                    'abstract_declarator: {
                        let (parse, r2) = AbstractDeclarator::parse(stack, r)?; 
                        let Some(l) = r2.first() else { break 'abstract_declarator; }; 
                        let lt = &l.token_type; 
                        let TokenType::Parenthesis { is_left: false } = lt else { break 'abstract_declarator; }; 
                        this = DirectAbstractDeclarator::AbstractDeclarator(Box::new(Ast(AstType::AbstractDeclarator(parse), &r[..r.len() - r2.len()]))); 
                        rst = &r2[1..];
                        break 'out; 
                    }
                    'param_type_list: {
                        let (parse, r2) = ParamTypeList::parse(stack, r)?; 
                        let Some(l) = r2.first() else { break 'param_type_list; }; 
                        let lt = &l.token_type; 
                        let TokenType::Parenthesis { is_left: false } = lt else { break 'param_type_list; }; 
                        this = DirectAbstractDeclarator::ParamTypeList {
                            direct_abstract_declarator: None, 
                            param_type_list: Box::new(Ast(AstType::ParamTypeList(parse), &r[..r.len() - r2.len()])), 
                        }; 
                        rst = &r2[1..];
                        break 'out;  
                    }
                    return Err(()); 
                }
                TokenType::Bracket { is_left: true } => {
                    let r = &rst[1..]; 
                    'empty: {
                        let Some(f) = r.first() else { break 'empty; }; 
                        let ft = &f.token_type; 
                        let TokenType::Bracket { is_left: false } = ft else { break 'empty; }; 
                        this = DirectAbstractDeclarator::EmptyBracket(None); 
                        rst = &r[1..];
                        break 'out; 
                    }
                    'const_exp: {
                        let (parse, r2) = AbstractDeclarator::parse(stack, r)?; 
                        let Some(l) = r2.first() else { break 'const_exp; }; 
                        let lt = &l.token_type; 
                        let TokenType::Bracket { is_left: false } = lt else { break 'const_exp; }; 
                        this = DirectAbstractDeclarator::ConstExp {
                            direct_abstract_declarator: None, 
                            const_exp: Box::new(Ast(AstType::AbstractDeclarator(parse), &r[..r.len() - r2.len()])), 
                        }; 
                        rst = &r2[1..];
                        break 'out;  
                    }
                    return Err(()); 
                }
                _ => return Err(()), 
            }
        } 
        loop {
            let Some(first) = rst.first() else { break }; 
            let ft = &first.token_type; 
            let r = &rst[1..]; 
            match ft {
                TokenType::Parenthesis { is_left: true } => {
                    let Some(f2) = r.first() else { break };  
                    let f2t = &f2.token_type; 
                    if let TokenType::Parenthesis { is_left: false } = f2t {
                        let ast = Ast(AstType::DirectAbstractDeclarator(this), &tokens[..tokens.len() - rst.len()]);
                        let t = DirectAbstractDeclarator::EmptyParenthesis(Some(Box::new(ast))); 
                        this = t;
                        rst = &r[1..];
                        continue ; 
                    } 
                    let p = ParamTypeList::parse(stack, r); 
                    let Ok(p) = p else { break }; 
                    let (parse, p2) = p; 
                    let Some(l) = p2.first() else { break }; 
                    let lt = &l.token_type; 
                    let TokenType::Parenthesis { is_left: false } = lt else { break }; 
                    let ast = Ast(AstType::DirectAbstractDeclarator(this), &tokens[..tokens.len() - rst.len()]); 
                    let ast2 = Ast(AstType::ParamTypeList(parse), &r[..r.len() - p2.len()]); 
                    this = DirectAbstractDeclarator::ParamTypeList {
                        direct_abstract_declarator: Some(Box::new(ast)), 
                        param_type_list: Box::new(ast2), 
                    }; 
                    rst = &p2[1..]; 
                }
                TokenType::Bracket { is_left: true } => {
                    let Some(f2) = r.first() else { break }; 
                    let f2t = &f2.token_type; 
                    if let TokenType::Bracket { is_left: false } = f2t {
                        let ast = Ast(AstType::DirectAbstractDeclarator(this), &tokens[..tokens.len() - rst.len()]);
                        let t = DirectAbstractDeclarator::EmptyBracket(Some(Box::new(ast))); 
                        this = t;
                        rst = &r[1..];
                        continue ; 
                    } 
                    let p = ConstExp::parse(stack, r); 
                    let Ok(p) = p else { break }; 
                    let (parse, p2) = p;
                    let Some(l) = p2.first() else { break }; 
                    let lt = &l.token_type; 
                    let TokenType::Bracket { is_left: false } = lt else { break }; 
                    let ast = Ast(AstType::DirectAbstractDeclarator(this), &tokens[..tokens.len() - rst.len()]); 
                    let ast2 = Ast(AstType::ConstExp(parse), &r[..r.len() - p2.len()]); 
                    this = DirectAbstractDeclarator::ConstExp {
                        direct_abstract_declarator: Some(Box::new(ast)), 
                        const_exp: Box::new(ast2), 
                    }; 
                    rst = &p2[1..]; 
                }
                _ => break, 
            }
        }
        return Ok((this, rst)); 
    } 
} 