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
}

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct StorageClassSpec <'a> (pub Token<'a> );

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TypeQualifier <'a> (pub Token<'a>); 

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
                    return Ok((DeclSpec::TypeSpec(Box::new(node)), rst));  
                }
            Token { token_type: TokenType::Keyword(k), .. } if 
                false || k == &Keyword::Enum => {
                    let node = r#enum::EnumSpec::parse(stack, tokens)?; 
                    let rst = node.1; 
                    let len = node.1.len(); 
                    let len = tokens.len() - len; 
                    let node = Ast(AstType::EnumSpec(node.0), &tokens[..len]); 
                    let ans = DeclSpec::TypeSpec(Box::new(node)); 
                    return Ok((ans, rst)); 
                } 
            _ => (), 
        }
        todo!()
    }

} 

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DeclSpecs <'a> ( pub Vec<Ast<'a>> ); 

impl <'a> Parser<'a> for DeclSpecs<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        todo!()
    }
    
} 