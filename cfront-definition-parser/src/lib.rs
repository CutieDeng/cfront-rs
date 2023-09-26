#![feature(never_type)]

use cfront_definition::token::Token;

pub struct Parser {
}

pub fn parser(input: &[Token<'_>]) -> Result<(), ()> {
    let _ = input; 
    Ok(())
} 

pub mod ast; 