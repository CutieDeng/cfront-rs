use std::fs::File;
use std::io::Read;

use cfront_definition_lexer;
use cfront_definition_parser; 

fn main() {
    // redirect input for file main.c
    let file = File::open("main.c").unwrap(); 
    let mut content = String::new(); 
    { file } .read_to_string(&mut content).unwrap(); 
    let tokens = cfront_definition_lexer::analyze(&content); 
    println!("{tokens:?}");
    let expressions = cfront_definition_parser::parser(&tokens); 
    println!("{:#?}", expressions);
    match expressions {
        Ok(_) => {
            // if end.is_empty() {
            println!("A expression! ")
            // } else {
            //     println!("Not a complete expression! ")
            // }
        }
        Err(_) => 
            println!("Error in parsing! ")
    }; 
}
