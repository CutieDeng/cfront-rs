use cfront_definition::token::{Token, TokenType};

use super::AstNode;

pub enum Expression <'a> {
    Primary(TokenType<'a>),
    CompoundPrimary(Box<AstNode<'a>>), 
}

