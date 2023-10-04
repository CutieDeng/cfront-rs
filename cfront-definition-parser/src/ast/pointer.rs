use cfront_definition::{token::{Token, TokenType}, Keyword};

use crate::{Parser, ast::{type_qualifier_list::TypeQualifierList, AstType}};

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pointer<'a> {
    pub type_qualifier_list: Option<Box<Ast<'a>>>, 
    pub pointer: Option<Box<Ast<'a>>>,
}

impl <'a> Parser<'a> for Pointer<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let p = tokens.first().ok_or(())?; 
        let TokenType::Operator("*") = p.token_type else { return Err(()) };
        let mut rst = &tokens[1..]; 
        let p2 = rst.first(); 
        let mut type_qualifier_list = None; 
        let mut pointer = None; 
        match p2 {
            Some(p2) => {
                match p2.token_type {
                    | TokenType::Keyword(Keyword::Const) 
                    | TokenType::Keyword(Keyword::Volatile) => {
                        let p = TypeQualifierList::parse(stack, rst)?; 
                        type_qualifier_list = Some(Box::new(Ast(AstType::TypeQualifierList(p.0), &rst[..rst.len() - p.1.len()]))); 
                        rst = p.1; 
                    }
                    _ => (), 
                }
            }
            None => (), 
        }
        let p3 = rst.first(); 
        match p3 {
            Some(p3) => {
                match p3.token_type {
                    TokenType::Operator("*") => {
                        let p = Pointer::parse(stack, rst)?; 
                        pointer = Some(Box::new(Ast(AstType::Pointer(p.0), &rst[..rst.len() - p.1.len()]))); 
                        rst = p.1; 
                    }
                    _ => (), 
                }
            }
            None => (),  
        }
        return Ok((Pointer {
            type_qualifier_list, 
            pointer, 
        }, rst));  
    } 
    
} 