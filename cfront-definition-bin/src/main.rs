

fn main() {
    let stdin = std::io::stdin(); 
    let mut content = String::new(); 
    stdin.read_line(&mut content).unwrap(); 
    let tokens = cfront_definition_lexer::analyze(&content); 
    let expressions = cfront_definition_parser::parser(&tokens); 
    // println!("{:#?}", expressions);
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
