use cfront_definition::{token::{Token, TokenType}, Keyword};

use crate::{ast::AstNode, Parser};

use super::r#struct::Struct;

pub struct StructSpec <'a> {
    pub r#struct: Struct, 
    pub identifier: Option<Token<'a>>, 
    pub struct_decl_list: !, 
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypeSpec <'a> {
    RawType ( Token<'a> ),
    StructSpec ( AstNode<'a> ), 
    EnumSpec (!), 
    TypedefName ( Token<'a> ),
}

impl <'a> Parser<'a> for TypeSpec<'a> {
    type E = (); 

    fn parse (tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser>::E> {
        let f = tokens.first().ok_or(())?; 
        let rest = &tokens[1..]; 
        match f.token_type {
            | TokenType::Keyword(Keyword::Void) 
            | TokenType::Keyword(Keyword::Char) 
            | TokenType::Keyword(Keyword::Short)
            | TokenType::Keyword(Keyword::Int)
            | TokenType::Keyword(Keyword::Long)
            | TokenType::Keyword(Keyword::Float)
            | TokenType::Keyword(Keyword::Double)
            | TokenType::Keyword(Keyword::Signed)
            | TokenType::Keyword(Keyword::Unsigned)
            => {
                let ans = TypeSpec::RawType(f.clone()); 
                return Ok((ans, rest));  
            }
            | TokenType::Identifier(_) => {
                let ans = TypeSpec::TypedefName(f.clone()); 
                return Ok((ans, rest));  
            }
            | TokenType::Keyword(Keyword::Struct) 
            | TokenType::Keyword(Keyword::Union) => {
                unimplemented!()
            }
            | TokenType::Keyword(Keyword::Enum) => {
                unimplemented!()
            } 
            _ => return Err(()),
        }
    }
}