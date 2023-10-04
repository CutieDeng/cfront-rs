use cfront_definition::token::Token;

use crate::{Parser, ast::decl_specs::TypeQualifier};

use super::{Ast, AstType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TypeQualifierList<'a> ( pub Vec<Ast<'a>> ); 

impl <'a> Parser<'a> for TypeQualifierList<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let mut rst = Vec::new(); 
        let mut rest = tokens; 
        loop {
            let p = TypeQualifier::parse(stack, rest); 
            let Ok(p) = p else { break }; 
            let len = rest.len() - p.1.len(); 
            let q = Ast(AstType::TypeQualifier(p.0), &rest[..len]); 
            rst.push(q);
            rest = p.1;
        }
        if rst.is_empty() {
            return Err(()); 
        } else {
            return Ok((TypeQualifierList(rst), rest)); 
        }
    }
}