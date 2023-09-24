use std::io::{stdin, Read};

use cfront::token::tokenize::tokenize;

pub fn main() {
    let mut buf = String::new(); 
    stdin().read_to_string(&mut buf).unwrap(); 
    let lexer = tokenize(&buf); 
    for l in lexer {
        println!("{:?}", l); 
    }
}