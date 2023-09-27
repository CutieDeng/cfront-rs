use cfront_definition::token::TokenType;

pub struct StorageClassSpec(pub TokenType<'static>);

pub fn p(i: &TokenType) -> StorageClassSpec {
    match i {
        | TokenType::Keyword(k) 
        => {
            return StorageClassSpec(TokenType::Keyword(k.clone())); 
        }
        _ => todo!()
    }
}