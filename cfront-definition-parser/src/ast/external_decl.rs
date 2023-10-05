use cfront_definition::token::Token;

use crate::ast::AstType;
use crate::ast::decl::Decl;
use crate::ast::function_definition::FunctionDefinition;
use crate::Parser;

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExternalDecl <'a> {
    FunctionDefinition(Box<Ast<'a>>),
    Decl(Box<Ast<'a>>),
}

impl <'a> Parser<'a> for ExternalDecl<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let fd = FunctionDefinition::parse(stack, tokens); 
        let d = Decl::parse(stack, tokens); 
        let select_fd; 
        match (&fd, &d) {
            (Ok((_, fd_l)), Ok((_, d_l))) => select_fd = fd_l.len() < d_l.len(),
            (Ok(_), Err(_)) => select_fd = true, 
            (Err(_), Ok(_)) => select_fd = false, 
            (Err(_), Err(_)) => return Err(()), 
        }
        if select_fd {
            let f = fd.unwrap(); 
            let len = tokens.len() - f.1.len(); 
            let a = Ast(AstType::FunctionDefinition(f.0), &tokens[..len]);
            return Ok((ExternalDecl::FunctionDefinition(Box::new(a)), f.1)); 
        } else {
            let d = d.unwrap(); 
            let len = tokens.len() - d.1.len(); 
            let a = Ast(AstType::Decl(d.0), &tokens[..len]); 
            return Ok((ExternalDecl::Decl(Box::new(a)), d.1)); 
        }
    }
} 
