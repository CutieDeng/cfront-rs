use cfront_definition::token::Token;

use crate::{Parser, ast::{struct_decl::SpecQualifierList, AstType, abstract_declarator::AbstractDeclarator}};

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TypeName <'a> {
    pub spec_qualifer_list: Box<Ast<'a>>, 
    pub abstract_declarator: Option<Box<Ast<'a>>>, 
}

impl <'a> Parser<'a> for TypeName<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let (s, r) = SpecQualifierList::parse(stack, tokens)?; 
        let s = Ast(AstType::SpecQualifierList(s), &tokens[..tokens.len() - r.len()]);
        let u = AbstractDeclarator::parse(stack, r);
        let mut rst = r;  
        let a = u.ok().map(|(u, r2)| {
            rst = r2; 
            Ast(AstType::AbstractDeclarator(u), &r[..r.len() - r2.len()])
        });
        let ans = TypeName {
            spec_qualifer_list: Box::new(s), 
            abstract_declarator: a.map(Box::new), 
        }; 
        return Ok((ans, rst)); 
    } 
}