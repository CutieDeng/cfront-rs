use cfront_definition::{token::{Token, TokenType}, Keyword};

use crate::{Parser, ast::{r#enum::EnumeratorList, AstType}};

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct StructOrUnionSpec <'a> {
    pub r#struct: Token<'a>, 
    pub identifier: Option<Token<'a>>,
    pub struct_decl_list: Option<Box<Ast<'a>>>, 
}

impl <'a> Parser<'a> for StructOrUnionSpec<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let f = tokens.first().ok_or(())?;
        let ft = &f.token_type;
        match ft {
            | TokenType::Keyword(Keyword::Struct) 
            | TokenType::Keyword(Keyword::Union) 
            => (),
            _ => return Err(()), 
        }
        let r#struct = f.clone(); 
        let rs = &tokens[1..]; 
        let mut identifier = None; 
        let mut struct_decl_list = None; 
        let mut rst = rs; 
        'id: {
            let f = rst.first().ok_or(())?;
            let ft = &f.token_type;
            match ft {
                TokenType::Identifier(_) => (),
                _ => break 'id, 
            }
            identifier = Some(f.clone()); 
            rst = &rst[1..]; 
        } 
        'lp: {
            let f = rst.first();
            let Some(f) = f else { break 'lp }; 
            let ft = &f.token_type; 
            match ft {
                TokenType::Brace { is_left: true } => (), 
                _ => break 'lp, 
            } 
            let p = EnumeratorList::parse(stack, rst);
            let Ok((p, r)) = p else { break 'lp }; 
            let f = r.first(); 
            let Some(f) = f else { break 'lp }; 
            let ft = &f.token_type; 
            match ft {
                TokenType::Brace { is_left: false } => (), 
                _ => break 'lp, 
            } 
            let len = rst.len() - r.len(); 
            let t = Ast(AstType::EnumeratorList(p), &rst[..len]); 
            struct_decl_list = Some(Box::new(t)); 
            rst = &r[1..];
        } 
        if identifier.is_none() && struct_decl_list.is_none() {
            return Err(()); 
        } 
        let ans = StructOrUnionSpec {
            r#struct, 
            identifier, 
            struct_decl_list, 
        }; 
        return Ok((ans, rst)); 
    }
    
} 