

fn main() {
    let stdin = std::io::stdin(); 
    let mut content = String::new(); 
    stdin.read_line(&mut content).unwrap(); 
    let tokens = cfront_definition_lexer::analyze(&content); 
    let expressions = cfront_definition_parser::ast::compound::parse_expression(&tokens); 
    println!("{:#?}", expressions);
}
