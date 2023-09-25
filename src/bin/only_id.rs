use std::fs::File;
use std::env::args;
use std::io::Read;

use cfront::{token::{tokenize::tokenize, Token}, TokenType};

pub fn main() {
    let mut a = args(); 
    let _ = a.next(); 
    let input_file = a.next().unwrap();
    let file = File::open(input_file).unwrap();  
    let mut input = String::new(); 
    {file}.read_to_string(&mut input).unwrap(); 
    let lexer = tokenize(&input); 
    let mut rst = Vec::new(); 
    for l in lexer {
        match l {
            Token {
                token_type: TokenType::Identifier(s), 
                line,
                column: _,
            } => {
                rst.push((line + 1, s)); 
            }
            _ => {},
        }
    }
    let size = rst.len(); 
    for (ll, st) in rst {
        println!("line {ll}: {st}");
    }
    if size != 0 {
        println!("There are {size} occurences of valid identifiers"); 
    } else {
        println!("No occruence of valid identifiers"); 
    }
}