use cfront_definition::{token::{TokenType, Token}, Keyword};

use crate::{Parser, ast::AstNode};

#[derive(Debug, PartialEq, Eq, Clone, )]
pub struct StructSpec <'a> {
    pub r#struct: Token<'a>,
    pub id: Option<Token<'a>>,
    pub struct_decl_list: Vec<AstNode<'a>>, 
}

impl <'a> Parser<'a> for StructSpec<'a> {
    type E = (); 

    fn parse (tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser>::E> {
        let first = tokens.first().ok_or(())?; 
        let r#struct; 
        match first.token_type {
            | TokenType::Keyword(Keyword::Struct) 
            | TokenType::Keyword(Keyword::Union) 
            => {
                r#struct = first.clone(); 
            }
            _ => return Err(()), 
        }
        let mut rst = &tokens[1..]; 
        let id = match rst.first().ok_or(())?.token_type {
            TokenType::Identifier(_) => {
                let id = rst.first().ok_or(())?.clone(); 
                rst = &rst[1..]; 
                Some(id)
            }
            _ => None, 
        }; 
        let par = rst.first(); 
        if id.is_none() && par.is_none() {
            return Err(()) 
        }
        let mut list = Vec::new(); 
        let par = par.ok_or(())?; 
        match par.token_type {
            TokenType::Brace { is_left : true } => {
                rst = &rst[1..]; 
                loop {
                    // let (decl, rst_) = AstNode::parse(rst)?; 
                    // rst = rst_; 
                    // list.push(decl); 
                    // let first = rst.first().ok_or(())?; 
                    // match first.token_type {
                    //     TokenType::Brace { is_left : false } => {
                    //         rst = &rst[1..]; 
                    //         break; 
                    //     }
                    //     TokenType::Comma => {
                    //         rst = &rst[1..]; 
                    //         continue; 
                    //     }
                    //     _ => return Err(()), 
                    // }
                    unimplemented!()
                } 

            }
            _ => (), 
        }
        return Ok((Self {
            r#struct, 
            id, 
            struct_decl_list: list, 
        }, rst))
    }
}

/// SpecQualiferList is a list of specifiers and qualifiers. 
pub struct SpecQualiferList <'a> ( pub Vec<AstNode<'a>>); 

pub struct StructDecl<'a> {
    a: &'a !
}