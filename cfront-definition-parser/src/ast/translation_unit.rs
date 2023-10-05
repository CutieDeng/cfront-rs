use cfront_definition::token::Token;

use crate::{Parser, ast::AstType};

use super::{Ast, external_decl::ExternalDecl};

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct TranslationUnit <'a> (pub Vec<Ast<'a>>); 

impl <'a> Parser<'a> for TranslationUnit<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, mut tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        _ = stack; 
        let mut ans = Vec::new(); 
        loop {
            let p = ExternalDecl::parse(stack, tokens);
            let Ok((p, r)) = p else { break }; 
            let len = tokens.len() - r.len(); 
            let a = Ast(AstType::ExternalDecl(p), &tokens[..len]); 
            ans.push(a); 
            tokens = r; 
        }
        if ans.is_empty() {
            return Err(()); 
        } else {
            return Ok((TranslationUnit(ans), tokens)); 
        }
    } 

}
