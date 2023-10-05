use cfront_definition::{token::{Token, TokenType}, Keyword};

use crate::{Parser, ast::AstType};

use super::{Ast, r#struct, r#enum};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DeclSpec <'a> {
    StorageClassSpec(Box<Ast<'a>>) , 
    TypeSpec(Box<Ast<'a>>), 
    TypeQualifier(Box<Ast<'a>>), 
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypeSpec <'a> {
    Raw(Token<'a>),
    StructOrUnionSpec(Box<Ast<'a>>), 
    EnumSpec(Box<Ast<'a>>), 
    TypedefName(Token<'a>),
}

impl <'a> Parser<'a> for TypeSpec<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let first = tokens.first().ok_or(())?; 
        let Token { token_type, .. } = first; 
        match token_type {
            | TokenType::Keyword(Keyword::Void)
            | TokenType::Keyword(Keyword::Char)
            | TokenType::Keyword(Keyword::Short)
            | TokenType::Keyword(Keyword::Int) 
            | TokenType::Keyword(Keyword::Long) 
            | TokenType::Keyword(Keyword::Float) 
            | TokenType::Keyword(Keyword::Double) 
            | TokenType::Keyword(Keyword::Signed) 
            | TokenType::Keyword(Keyword::Unsigned) => {
                let node = TypeSpec::Raw(first.clone()); 
                return Ok((node, &tokens[1..])); 
            } 
            | TokenType::Keyword(Keyword::Struct) 
            | TokenType::Keyword(Keyword::Union) => {
                let node = r#struct::StructOrUnionSpec::parse(stack, tokens)?; 
                let rst = node.1 ;
                let len = tokens.len() - rst.len(); 
                let node = Ast(AstType::StructOrUnionSpec(node.0), &tokens[..len]); 
                return Ok((TypeSpec::StructOrUnionSpec(Box::new(node)), rst)); 
            }
            | TokenType::Keyword(Keyword::Enum) => {
                let node = r#enum::EnumSpec::parse(stack, tokens)?; 
                let rst = node.1; 
                let len = tokens.len() - rst.len(); 
                let node = Ast(AstType::EnumSpec(node.0), &tokens[..len]); 
                return Ok((TypeSpec::EnumSpec(Box::new(node)), rst));  
            }
            | TokenType::Identifier(_) => {
                // TODO: realize the typedef recognization, now deprecates it. 
                if false {
                    // preview next to check is '(' or '[' or not. if yes, then drop it. 
                    let rs = tokens.get(1);
                    if let Some(Token { token_type: TokenType::Parenthesis { is_left: true }, .. }) = rs {
                        return Err(()); 
                    } else if let Some(Token { token_type: TokenType::Bracket { is_left: true }, .. }) = rs {
                        return Err(()); 
                    }
                    let node = TypeSpec::TypedefName(first.clone()); 
                    return Ok((node, &tokens[1..])); 
                }
                return Err(()); 
            }
            _ => return Err(()), 
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct StorageClassSpec <'a> (pub Token<'a> );

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TypeQualifier <'a> (pub Token<'a>); 

impl <'a> Parser<'a> for TypeQualifier<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        _ = stack;
        let first = tokens.first().ok_or(())?;
        let Token { token_type, .. } = first; 
        match token_type {
            | TokenType::Keyword(Keyword::Const)
            | TokenType::Keyword(Keyword::Volatile) => {
                let node = TypeQualifier(first.clone()); 
                return Ok((node, &tokens[1..])); 
            }
            _ => return Err(()), 
        }
    }
}

impl <'a> Parser<'a> for DeclSpec<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let first = tokens.first().ok_or(())?; 
        match first {
            Token { token_type: TokenType::Keyword(k), .. } if 
                false || k == &Keyword::Auto || k == &Keyword::Register 
                || k == &Keyword::Static || k == &Keyword::Extern 
                || k == &Keyword::Typedef => {
                    let node = StorageClassSpec(first.clone()); 
                    let node = Ast(AstType::StorageClassSpec(node), &tokens[..1]); 
                    let ans = DeclSpec::StorageClassSpec(Box::new(node)); 
                    return Ok((ans, &tokens[1..])); 
                } 
            Token { token_type: TokenType::Keyword(k), .. } if 
                false || k == &Keyword::Void || k == &Keyword::Char 
                || k == &Keyword::Short || k == &Keyword::Int 
                || k == &Keyword::Long || k == &Keyword::Float 
                || k == &Keyword::Double || k == &Keyword::Signed 
                || k == &Keyword::Unsigned => {
                    let node = TypeSpec::Raw(first.clone()); 
                    let node = Ast(AstType::TypeSpec(node), &tokens[..1]); 
                    let ans = DeclSpec::TypeSpec(Box::new(node)); 
                    return Ok((ans, &tokens[1..])); 
                } 
            Token { token_type: TokenType::Keyword(k), .. } if 
                false || k == &Keyword::Const || k == &Keyword::Volatile => {
                    let node = TypeQualifier(first.clone()); 
                    let node = Ast(AstType::TypeQualifier(node), &tokens[..1]); 
                    let ans = DeclSpec::TypeQualifier(Box::new(node)); 
                    return Ok((ans, &tokens[1..])); 
                } 
            Token { token_type: TokenType::Keyword(k), .. } if 
                false || k == &Keyword::Struct || k == &Keyword::Union => {
                    let node = r#struct::StructOrUnionSpec::parse(stack, tokens)?; 
                    let rst = node.1; 
                    let len = node.1.len(); 
                    let len = tokens.len() - len; 
                    let node = Ast(AstType::StructOrUnionSpec(node.0), &tokens[..len]); 
                    let node = TypeSpec::StructOrUnionSpec(Box::new(node)); 
                    let node = Ast(AstType::TypeSpec(node), &tokens[..len]);
                    let ans = DeclSpec::TypeSpec(Box::new(node)); 
                    return Ok((ans, rst));  
                }
            Token { token_type: TokenType::Keyword(k), .. } if 
                false || k == &Keyword::Enum => {
                    let node = r#enum::EnumSpec::parse(stack, tokens)?; 
                    let rst = node.1; 
                    let len = node.1.len(); 
                    let len = tokens.len() - len; 
                    let node = Ast(AstType::EnumSpec(node.0), &tokens[..len]); 
                    let node = TypeSpec::EnumSpec(Box::new(node)); 
                    let node = Ast(AstType::TypeSpec(node), &tokens[..len]); 
                    let ans = DeclSpec::TypeSpec(Box::new(node)); 
                    return Ok((ans, rst)); 
                } 
            Token { token_type: TokenType::Identifier(_), .. } => {
                let t = TypeSpec::parse(stack, tokens)?;
                let node = Ast(AstType::TypeSpec(t.0), &tokens[..tokens.len() - t.1.len()]); 
                let ans = DeclSpec::TypeSpec(Box::new(node)); 
                return Ok((ans, t.1)); 
            }
            _ => return Err(()),
        }
    }

} 

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DeclSpecs <'a> ( pub Vec<Ast<'a>> ); 

impl <'a> Parser<'a> for DeclSpecs<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let mut rst = tokens ;
        let mut ans = Vec::new(); 
        loop {
            let p = DeclSpec::parse(stack, rst); 
            let Ok(node) = p else {
                if ans.is_empty() {
                    return Err(()); 
                }
                return Ok((DeclSpecs(ans), rst)); 
            }; 
            let len = rst.len(); 
            rst = node.1; 
            let len = len - rst.len(); 
            let node = Ast(AstType::DeclSpec(node.0), &rst[..len]); 
            ans.push(node); 
        } 
    }
    
} 